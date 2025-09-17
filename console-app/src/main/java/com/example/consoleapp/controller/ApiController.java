package com.example.consoleapp.controller;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Locale;
import java.util.Map;
import java.util.stream.Collectors;

import org.springframework.http.MediaType;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequestMapping(path = "/api", produces = MediaType.APPLICATION_JSON_VALUE)
public class ApiController {

    public record User(Long id, String name, String email, String role) {}

    private final List<User> users;

    public ApiController() {
        List<User> seed = new ArrayList<>();
        seed.add(new User(1L, "Alice Johnson", "alice@example.com", "ADMIN"));
        seed.add(new User(2L, "Bob Smith", "bob@example.com", "USER"));
        seed.add(new User(3L, "Carol Davis", "carol@example.com", "USER"));
        seed.add(new User(4L, "David Lee", "david@example.com", "ANALYST"));
        seed.add(new User(5L, "Eve Martinez", "eve@example.com", "USER"));
        this.users = List.copyOf(seed);
    }

    @GetMapping("/dashboard")
    public Map<String, Object> getDashboard() {
        Map<String, Object> data = new HashMap<>();
        data.put("totalUsers", users.size());
        data.put("activeUsers", 3);
        data.put("openAlerts", 2);
        data.put("systemHealth", "OK");
        return data;
    }

    @GetMapping("/users")
    public List<User> getUsers(@RequestParam(name = "query", required = false, defaultValue = "") String query) {
        String q = query == null ? "" : query.toLowerCase(Locale.ROOT).trim();
        if (q.isEmpty()) {
            return users;
        }
        return users.stream()
            .filter(u -> containsIgnoreCase(u.name(), q) || containsIgnoreCase(u.email(), q) || containsIgnoreCase(u.role(), q))
            .collect(Collectors.toList());
    }

    private boolean containsIgnoreCase(String value, String needle) {
        return value != null && value.toLowerCase(Locale.ROOT).contains(needle);
    }
}

