package com.example.excel2mongo;

import org.apache.poi.ss.usermodel.*;
import org.apache.poi.ss.usermodel.Row.MissingCellPolicy;
import org.apache.poi.ss.util.NumberToTextConverter;

import java.io.FileInputStream;
import java.io.IOException;
import java.io.InputStream;
import java.math.BigDecimal;
import java.math.RoundingMode;
import java.time.Instant;
import java.time.format.DateTimeFormatter;
import java.util.*;

public final class ExcelToMongoScriptGenerator {
    private ExcelToMongoScriptGenerator() {}

    public static String generateMongoInsertScript(String filePath) throws IOException {
        StringBuilder scriptBuilder = new StringBuilder();
        try (InputStream inputStream = new FileInputStream(filePath); Workbook workbook = WorkbookFactory.create(inputStream)) {
            FormulaEvaluator evaluator = workbook.getCreationHelper().createFormulaEvaluator();
            int numberOfSheets = workbook.getNumberOfSheets();
            for (int s = 0; s < numberOfSheets; s++) {
                Sheet sheet = workbook.getSheetAt(s);
                if (sheet == null) {
                    continue;
                }
                String originalSheetName = sheet.getSheetName();
                String collectionName = sanitizeCollectionName(originalSheetName);

                Row headerRow = findFirstNonEmptyRow(sheet);
                if (headerRow == null) {
                    continue;
                }
                List<String> headers = extractHeaders(headerRow);
                if (headers.isEmpty()) {
                    continue;
                }

                List<Map<String, Object>> documents = new ArrayList<Map<String, Object>>();
                for (int r = headerRow.getRowNum() + 1; r <= sheet.getLastRowNum(); r++) {
                    Row row = sheet.getRow(r);
                    if (row == null) {
                        continue;
                    }
                    Map<String, Object> doc = new LinkedHashMap<String, Object>();
                    boolean allNull = true;
                    for (int c = 0; c < headers.size(); c++) {
                        String header = headers.get(c);
                        if (header == null || header.isEmpty()) {
                            continue;
                        }
                        Cell cell = row.getCell(c, MissingCellPolicy.RETURN_BLANK_AS_NULL);
                        Object value = readCellValue(cell, evaluator);
                        if (value != null) {
                            allNull = false;
                        }
                        doc.put(header, value);
                    }
                    if (!allNull) {
                        documents.add(doc);
                    }
                }

                if (!documents.isEmpty()) {
                    appendInsertMany(scriptBuilder, collectionName, documents);
                    if (s < numberOfSheets - 1) {
                        scriptBuilder.append("\n\n");
                    }
                }
            }
        } catch (RuntimeException ex) {
            // Re-throw with a simpler message to avoid verbose POI internals
            throw new IOException("Failed to parse Excel file: " + ex.getMessage(), ex);
        }
        return scriptBuilder.toString();
    }

    private static Row findFirstNonEmptyRow(Sheet sheet) {
        for (int r = sheet.getFirstRowNum(); r <= sheet.getLastRowNum(); r++) {
            Row row = sheet.getRow(r);
            if (row == null) {
                continue;
            }
            if (!isRowEmpty(row)) {
                return row;
            }
        }
        return null;
    }

    private static boolean isRowEmpty(Row row) {
        short lastCell = row.getLastCellNum();
        if (lastCell < 0) {
            return true;
        }
        for (int c = 0; c < lastCell; c++) {
            Cell cell = row.getCell(c, MissingCellPolicy.RETURN_BLANK_AS_NULL);
            if (cell != null && getCellNonEmptyString(cell).length() > 0) {
                return false;
            }
        }
        return true;
    }

    private static String getCellNonEmptyString(Cell cell) {
        if (cell == null) {
            return "";
        }
        switch (cell.getCellType()) {
            case STRING:
                return cell.getStringCellValue() != null ? cell.getStringCellValue().trim() : "";
            case BOOLEAN:
                return String.valueOf(cell.getBooleanCellValue());
            case NUMERIC:
                return NumberToTextConverter.toText(cell.getNumericCellValue());
            case FORMULA:
                return cell.getCellFormula() != null ? cell.getCellFormula().trim() : "";
            default:
                return "";
        }
    }

    private static List<String> extractHeaders(Row headerRow) {
        List<String> headers = new ArrayList<String>();
        int width = headerRow.getLastCellNum();
        for (int c = 0; c < width; c++) {
            Cell cell = headerRow.getCell(c, MissingCellPolicy.RETURN_BLANK_AS_NULL);
            String raw = cell != null ? cell.toString() : null;
            String header = raw != null ? raw.trim() : "";
            header = sanitizeFieldName(header);
            if (header.isEmpty()) {
                header = "column_" + (c + 1);
            }
            headers.add(header);
        }
        // Trim trailing empty headers
        int lastNonEmpty = headers.size() - 1;
        while (lastNonEmpty >= 0 && (headers.get(lastNonEmpty) == null || headers.get(lastNonEmpty).isEmpty())) {
            lastNonEmpty--;
        }
        if (lastNonEmpty + 1 < headers.size()) {
            headers = new ArrayList<String>(headers.subList(0, lastNonEmpty + 1));
        }
        return headers;
    }

    private static Object readCellValue(Cell cell, FormulaEvaluator evaluator) {
        if (cell == null) {
            return null;
        }
        CellType type = cell.getCellType();
        if (type == CellType.FORMULA) {
            CellValue evaluated = evaluator.evaluate(cell);
            if (evaluated == null) {
                return null;
            }
            switch (evaluated.getCellType()) {
                case BOOLEAN:
                    return evaluated.getBooleanValue();
                case NUMERIC:
                    if (DateUtil.isCellDateFormatted(cell)) {
                        Date date = cell.getDateCellValue();
                        return formatDateAsIso(date);
                    }
                    return bigDecimalFromDouble(evaluated.getNumberValue());
                case STRING:
                    return parseStringMaybeJson(evaluated.getStringValue());
                default:
                    return null;
            }
        }
        switch (type) {
            case BOOLEAN:
                return cell.getBooleanCellValue();
            case NUMERIC:
                if (DateUtil.isCellDateFormatted(cell)) {
                    Date date = cell.getDateCellValue();
                    return formatDateAsIso(date);
                }
                return bigDecimalFromDouble(cell.getNumericCellValue());
            case STRING:
                return parseStringMaybeJson(cell.getStringCellValue());
            case BLANK:
            default:
                return null;
        }
    }

    private static Object parseStringMaybeJson(String value) {
        if (value == null) {
            return null;
        }
        String trimmed = value.trim();
        if (trimmed.isEmpty()) {
            return null;
        }
        char first = trimmed.charAt(0);
        if ((first == '{' || first == '[')) {
            try {
                SimpleJsonParser parser = new SimpleJsonParser(trimmed);
                Object parsed = parser.parse();
                if (parser.isFullyConsumed()) {
                    return parsed;
                }
            } catch (IllegalArgumentException ignored) {
                // fall back to plain string
            }
        }
        return trimmed;
    }

    private static String sanitizeCollectionName(String name) {
        if (name == null) {
            return "Sheet";
        }
        String cleaned = name.trim().replaceAll("[^A-Za-z0-9_]+", "_");
        if (cleaned.isEmpty()) {
            cleaned = "Sheet";
        }
        if (Character.isDigit(cleaned.charAt(0))) {
            cleaned = "_" + cleaned;
        }
        return cleaned;
    }

    private static String sanitizeFieldName(String name) {
        if (name == null) {
            return "";
        }
        String cleaned = name.trim();
        cleaned = cleaned.replace('.', '_');
        if (!cleaned.isEmpty() && cleaned.charAt(0) == '$') {
            cleaned = "_" + cleaned.substring(1);
        }
        return cleaned;
    }

    private static BigDecimal bigDecimalFromDouble(double d) {
        return new BigDecimal(Double.toString(d));
    }

    private static String formatDateAsIso(Date date) {
        if (date == null) {
            return null;
        }
        Instant instant = date.toInstant();
        return DateTimeFormatter.ISO_INSTANT.format(instant);
    }

    private static void appendInsertMany(StringBuilder out, String collectionName, List<Map<String, Object>> documents) {
        out.append("db.").append(collectionName).append(".insertMany([\n");
        for (int i = 0; i < documents.size(); i++) {
            Map<String, Object> doc = documents.get(i);
            out.append("  {");
            int j = 0;
            for (Map.Entry<String, Object> entry : doc.entrySet()) {
                if (j++ > 0) {
                    out.append(",");
                }
                out.append("\"")
                   .append(escapeJsString(entry.getKey()))
                   .append("\": ");
                appendValue(out, entry.getValue());
            }
            out.append("}");
            if (i < documents.size() - 1) {
                out.append(",");
            }
            out.append("\n");
        }
        out.append("])\n");
    }

    @SuppressWarnings("unchecked")
    private static void appendValue(StringBuilder out, Object value) {
        if (value == null) {
            out.append("null");
            return;
        }
        if (value instanceof String) {
            out.append('"').append(escapeJsString((String) value)).append('"');
            return;
        }
        if (value instanceof Boolean) {
            out.append(((Boolean) value) ? "true" : "false");
            return;
        }
        if (value instanceof BigDecimal) {
            BigDecimal bd = ((BigDecimal) value).stripTrailingZeros();
            String s = bd.toPlainString();
            out.append(s);
            return;
        }
        if (value instanceof Number) {
            BigDecimal bd = new BigDecimal(value.toString());
            bd = bd.stripTrailingZeros();
            out.append(bd.toPlainString());
            return;
        }
        if (value instanceof Map) {
            Map<String, Object> map = (Map<String, Object>) value;
            out.append("{");
            int i = 0;
            for (Map.Entry<String, Object> e : map.entrySet()) {
                if (i++ > 0) out.append(",");
                out.append('"').append(escapeJsString(sanitizeFieldName(e.getKey()))).append('"').append(": ");
                appendValue(out, e.getValue());
            }
            out.append("}");
            return;
        }
        if (value instanceof List) {
            List<Object> list = (List<Object>) value;
            out.append("[");
            for (int i = 0; i < list.size(); i++) {
                if (i > 0) out.append(",");
                appendValue(out, list.get(i));
            }
            out.append("]");
            return;
        }
        // Fallback to string
        out.append('"').append(escapeJsString(String.valueOf(value))).append('"');
    }

    private static String escapeJsString(String s) {
        StringBuilder b = new StringBuilder((int) (s.length() * 1.2));
        for (int i = 0; i < s.length(); i++) {
            char ch = s.charAt(i);
            switch (ch) {
                case '"': b.append("\\\""); break;
                case '\\': b.append("\\\\"); break;
                case '\n': b.append("\\n"); break;
                case '\r': b.append("\\r"); break;
                case '\t': b.append("\\t"); break;
                case '\b': b.append("\\b"); break;
                case '\f': b.append("\\f"); break;
                default:
                    if (ch < 0x20) {
                        String hex = Integer.toHexString(ch);
                        b.append("\\u");
                        for (int k = hex.length(); k < 4; k++) b.append('0');
                        b.append(hex);
                    } else {
                        b.append(ch);
                    }
            }
        }
        return b.toString();
    }

    // Minimal JSON parser sufficient for nested objects/arrays
    private static final class SimpleJsonParser {
        private final String input;
        private int index;

        SimpleJsonParser(String input) {
            this.input = input;
            this.index = 0;
        }

        Object parse() {
            skipWhitespace();
            Object value = parseValue();
            skipWhitespace();
            return value;
        }

        boolean isFullyConsumed() {
            skipWhitespace();
            return index >= input.length();
        }

        private Object parseValue() {
            skipWhitespace();
            if (index >= input.length()) {
                throw error("Unexpected end of input");
            }
            char ch = input.charAt(index);
            switch (ch) {
                case '"':
                    return parseString();
                case '{':
                    return parseObject();
                case '[':
                    return parseArray();
                case 't':
                    expectLiteral("true");
                    return Boolean.TRUE;
                case 'f':
                    expectLiteral("false");
                    return Boolean.FALSE;
                case 'n':
                    expectLiteral("null");
                    return null;
                default:
                    if (ch == '-' || (ch >= '0' && ch <= '9')) {
                        return parseNumber();
                    }
                    throw error("Unexpected character '" + ch + "'");
            }
        }

        private Map<String, Object> parseObject() {
            Map<String, Object> map = new LinkedHashMap<String, Object>();
            expect('{');
            skipWhitespace();
            if (peek('}')) {
                expect('}');
                return map;
            }
            while (true) {
                skipWhitespace();
                String key = parseString();
                skipWhitespace();
                expect(':');
                skipWhitespace();
                Object value = parseValue();
                map.put(key, value);
                skipWhitespace();
                if (peek('}')) {
                    expect('}');
                    break;
                }
                expect(',');
            }
            return map;
        }

        private List<Object> parseArray() {
            List<Object> list = new ArrayList<Object>();
            expect('[');
            skipWhitespace();
            if (peek(']')) {
                expect(']');
                return list;
            }
            while (true) {
                skipWhitespace();
                Object value = parseValue();
                list.add(value);
                skipWhitespace();
                if (peek(']')) {
                    expect(']');
                    break;
                }
                expect(',');
            }
            return list;
        }

        private String parseString() {
            expect('"');
            StringBuilder sb = new StringBuilder();
            while (index < input.length()) {
                char ch = input.charAt(index++);
                if (ch == '"') {
                    return sb.toString();
                }
                if (ch == '\\') {
                    if (index >= input.length()) throw error("Unterminated escape sequence");
                    char esc = input.charAt(index++);
                    switch (esc) {
                        case '"': sb.append('"'); break;
                        case '\\': sb.append('\\'); break;
                        case '/': sb.append('/'); break;
                        case 'b': sb.append('\b'); break;
                        case 'f': sb.append('\f'); break;
                        case 'n': sb.append('\n'); break;
                        case 'r': sb.append('\r'); break;
                        case 't': sb.append('\t'); break;
                        case 'u':
                            if (index + 4 > input.length()) throw error("Invalid unicode escape");
                            int codePoint = 0;
                            for (int i = 0; i < 4; i++) {
                                char hex = input.charAt(index++);
                                int digit = Character.digit(hex, 16);
                                if (digit < 0) throw error("Invalid hex digit: " + hex);
                                codePoint = (codePoint << 4) + digit;
                            }
                            sb.append((char) codePoint);
                            break;
                        default:
                            throw error("Invalid escape: \\" + esc);
                    }
                } else {
                    sb.append(ch);
                }
            }
            throw error("Unterminated string");
        }

        private BigDecimal parseNumber() {
            int start = index;
            if (peek('-')) index++;
            if (peek('0')) {
                index++;
            } else if (isDigit1to9(peekChar())) {
                while (isDigit(peekChar())) index++;
            } else {
                throw error("Invalid number");
            }
            if (peek('.')) {
                index++;
                if (!isDigit(peekChar())) throw error("Invalid fraction");
                while (isDigit(peekChar())) index++;
            }
            if (peek('e') || peek('E')) {
                index++;
                if (peek('+') || peek('-')) index++;
                if (!isDigit(peekChar())) throw error("Invalid exponent");
                while (isDigit(peekChar())) index++;
            }
            String num = input.substring(start, index);
            try {
                return new BigDecimal(num);
            } catch (NumberFormatException e) {
                throw error("Invalid number: " + num);
            }
        }

        private void expectLiteral(String literal) {
            for (int i = 0; i < literal.length(); i++) {
                if (index + i >= input.length() || input.charAt(index + i) != literal.charAt(i)) {
                    throw error("Expected '" + literal + "'");
                }
            }
            index += literal.length();
        }

        private void expect(char ch) {
            if (index >= input.length() || input.charAt(index) != ch) {
                throw error("Expected '" + ch + "'");
            }
            index++;
        }

        private boolean peek(char ch) {
            return index < input.length() && input.charAt(index) == ch;
        }

        private char peekChar() {
            return index < input.length() ? input.charAt(index) : '\0';
        }

        private void skipWhitespace() {
            while (index < input.length()) {
                char ch = input.charAt(index);
                if (ch == ' ' || ch == '\n' || ch == '\r' || ch == '\t') {
                    index++;
                } else {
                    break;
                }
            }
        }

        private IllegalArgumentException error(String message) {
            return new IllegalArgumentException(message + " at index " + index);
        }
    }
}
