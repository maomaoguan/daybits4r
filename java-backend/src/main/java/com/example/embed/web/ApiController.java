package com.example.embed.web;

import org.springframework.http.MediaType;
import org.springframework.web.bind.annotation.*;

import java.util.Map;

@RestController
@RequestMapping("/api")
public class ApiController {

    @GetMapping("/ping")
    public Map<String, Object> ping() {
        return Map.of("ok", true, "message", "pong");
    }

    @PostMapping(value = "/echo", consumes = MediaType.APPLICATION_JSON_VALUE)
    public Map<String, Object> echo(@RequestBody Map<String, Object> body) {
        return Map.of("ok", true, "data", body);
    }

    @PostMapping(value = "/sum", consumes = MediaType.APPLICATION_JSON_VALUE)
    public Map<String, Object> sum(@RequestBody Map<String, Object> body) {
        Number a = (Number) body.getOrDefault("a", 0);
        Number b = (Number) body.getOrDefault("b", 0);
        return Map.of("ok", true, "data", a.doubleValue() + b.doubleValue());
    }
}

