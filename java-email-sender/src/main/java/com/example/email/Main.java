package com.example.email;

import java.util.List;
import java.util.Map;

public final class Main {
    public static void main(String[] args) {
        String template = "dear {user}, welcome to {}, we are looking forward to see you soon.";

        // Named and positional mix: {user} and {} for organization
        String user = envOrDefault("APP_USER", "Alice");
        String org = envOrDefault("APP_ORG", "Acme Inc");

        String subject = "Welcome to " + org;
        String body = TemplateEngine.renderNamed(template, Map.of("user", user));
        body = TemplateEngine.renderPositional(body, List.of(org));

        String smtpHost = requireEnv("SMTP_HOST");
        int smtpPort = Integer.parseInt(envOrDefault("SMTP_PORT", "587"));
        boolean useTls = Boolean.parseBoolean(envOrDefault("SMTP_TLS", "true"));
        String smtpUser = System.getenv("SMTP_USER");
        String smtpPass = System.getenv("SMTP_PASS");
        String from = requireEnv("SMTP_FROM");
        String to = requireEnv("MAIL_TO");

        EmailClient client = EmailClient.createSmtp(smtpHost, smtpPort, useTls, smtpUser, smtpPass, from);
        client.send(to, subject, body);
        System.out.println("Email sent to: " + to);
    }

    private static String envOrDefault(String key, String def) {
        String v = System.getenv(key);
        return v == null || v.isBlank() ? def : v;
    }

    private static String requireEnv(String key) {
        String v = System.getenv(key);
        if (v == null || v.isBlank()) {
            throw new IllegalArgumentException("Missing required env var: " + key);
        }
        return v;
    }
}
