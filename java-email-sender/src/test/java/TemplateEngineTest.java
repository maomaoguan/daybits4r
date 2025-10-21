package com.example.email;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

import java.util.List;
import java.util.Map;

public class TemplateEngineTest {
    @Test
    void rendersNamedAndPositional() {
        String template = "dear {user}, welcome to {}, we are looking forward to see you soon.";
        String named = TemplateEngine.renderNamed(template, Map.of("user", "Bob"));
        String rendered = TemplateEngine.renderPositional(named, List.of("Acme"));
        assertEquals("dear Bob, welcome to Acme, we are looking forward to see you soon.", rendered);
    }
}
