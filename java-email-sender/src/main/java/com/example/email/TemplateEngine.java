package com.example.email;

import java.util.List;
import java.util.Map;
import java.util.Objects;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public final class TemplateEngine {
    private static final Pattern NAMED_PLACEHOLDER = Pattern.compile("\\{([a-zA-Z_][a-zA-Z0-9_]*)\\}");
    private static final Pattern POSITIONAL_PLACEHOLDER = Pattern.compile("\\{\\}");

    private TemplateEngine() {}

    public static String renderNamed(String template, Map<String, ?> values) {
        Objects.requireNonNull(template, "template");
        Objects.requireNonNull(values, "values");

        Matcher matcher = NAMED_PLACEHOLDER.matcher(template);
        StringBuffer result = new StringBuffer();
        while (matcher.find()) {
            String key = matcher.group(1);
            Object value = values.get(key);
            if (value == null) {
                throw new IllegalArgumentException("Missing value for key: " + key);
            }
            String replacement = Matcher.quoteReplacement(String.valueOf(value));
            matcher.appendReplacement(result, replacement);
        }
        matcher.appendTail(result);
        return result.toString();
    }

    public static String renderPositional(String template, List<?> values) {
        Objects.requireNonNull(template, "template");
        Objects.requireNonNull(values, "values");

        Matcher matcher = POSITIONAL_PLACEHOLDER.matcher(template);
        StringBuffer result = new StringBuffer();
        int index = 0;
        while (matcher.find()) {
            if (index >= values.size()) {
                throw new IllegalArgumentException("Not enough positional values provided");
            }
            Object value = values.get(index++);
            String replacement = Matcher.quoteReplacement(String.valueOf(value));
            matcher.appendReplacement(result, replacement);
        }
        if (index < values.size()) {
            throw new IllegalArgumentException("Too many positional values provided: " + (values.size() - index));
        }
        matcher.appendTail(result);
        return result.toString();
    }
}
