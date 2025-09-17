package com.example.embed;

import com.sun.net.httpserver.Headers;
import com.sun.net.httpserver.HttpExchange;
import com.sun.net.httpserver.HttpHandler;
import com.sun.net.httpserver.HttpServer;

import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.net.InetSocketAddress;
import java.nio.charset.StandardCharsets;
import java.util.Map;

public class SimpleServer {
    public static void main(String[] args) throws Exception {
        int port = 8080;
        HttpServer server = HttpServer.create(new InetSocketAddress(port), 0);

        server.createContext("/", new StaticHandler());
        server.createContext("/api/ping", exchange -> {
            withHeaders(exchange.getResponseHeaders());
            String body = "{\"ok\":true,\"message\":\"pong\"}";
            respond(exchange, 200, body);
        });
        server.createContext("/api/echo", exchange -> {
            withHeaders(exchange.getResponseHeaders());
            if (!"POST".equals(exchange.getRequestMethod())) { exchange.sendResponseHeaders(405, -1); return; }
            String body = new String(exchange.getRequestBody().readAllBytes(), StandardCharsets.UTF_8);
            respond(exchange, 200, "{\"ok\":true,\"data\":" + (body.isEmpty()?"{}":body) + "}");
        });
        server.createContext("/api/sum", exchange -> {
            withHeaders(exchange.getResponseHeaders());
            if (!"POST".equals(exchange.getRequestMethod())) { exchange.sendResponseHeaders(405, -1); return; }
            String body = new String(exchange.getRequestBody().readAllBytes(), StandardCharsets.UTF_8);
            double a = extractNumber(body, "a");
            double b = extractNumber(body, "b");
            String out = String.format("{\"ok\":true,\"data\":%s}", a + b);
            respond(exchange, 200, out);
        });

        System.out.println("Java SimpleServer listening at http://localhost:" + port + "/host.html");
        server.start();
    }

    static void withHeaders(Headers headers) {
        headers.add("Content-Type", "application/json; charset=utf-8");
        headers.add("Access-Control-Allow-Origin", "*");
        headers.add("X-Frame-Options", "ALLOWALL");
        headers.add("Content-Security-Policy", "frame-ancestors 'self' *");
    }

    static void respond(HttpExchange exchange, int code, String body) throws IOException {
        byte[] bytes = body.getBytes(StandardCharsets.UTF_8);
        exchange.sendResponseHeaders(code, bytes.length);
        try (OutputStream os = exchange.getResponseBody()) { os.write(bytes); }
    }

    static class StaticHandler implements HttpHandler {
        @Override public void handle(HttpExchange exchange) throws IOException {
            String path = exchange.getRequestURI().getPath();
            if (path.equals("/")) path = "/host.html";
            InputStream in = this.getClass().getResourceAsStream("/static" + path);
            if (in == null) { exchange.sendResponseHeaders(404, -1); return; }

            Headers headers = exchange.getResponseHeaders();
            if (path.endsWith(".html")) headers.add("Content-Type", "text/html; charset=utf-8");
            else if (path.endsWith(".js")) headers.add("Content-Type", "application/javascript; charset=utf-8");
            else if (path.endsWith(".css")) headers.add("Content-Type", "text/css; charset=utf-8");
            headers.add("X-Frame-Options", "ALLOWALL");
            headers.add("Content-Security-Policy", "frame-ancestors 'self' *");

            byte[] bytes = in.readAllBytes();
            exchange.sendResponseHeaders(200, bytes.length);
            try (OutputStream os = exchange.getResponseBody()) { os.write(bytes); }
        }
    }

    static double extractNumber(String json, String key) {
        try {
            String pattern = "\"" + key + "\"";
            int i = json.indexOf(pattern);
            if (i < 0) return 0.0;
            int colon = json.indexOf(':', i);
            if (colon < 0) return 0.0;
            int end = colon + 1;
            while (end < json.length() && Character.isWhitespace(json.charAt(end))) end++;
            int j = end;
            while (j < json.length() && "0123456789+-.eE".indexOf(json.charAt(j)) >= 0) j++;
            return Double.parseDouble(json.substring(end, j));
        } catch (Exception e) {
            return 0.0;
        }
    }
}

