package com.example.excel2mongo;

import org.apache.poi.ss.usermodel.*;
import org.apache.poi.xssf.usermodel.XSSFWorkbook;
import org.apache.poi.hssf.usermodel.HSSFWorkbook;

import java.io.*;
import java.nio.charset.StandardCharsets;
import java.util.*;

/**
 * Excel to MongoDB insert script generator.
 *
 * Usage:
 *   java -jar excel-to-mongo-script-1.0.0-jar-with-dependencies.jar \
 *     --input /path/data.xlsx \
 *     --db mydb \
 *     --collection users \
 *     --output /path/insert_users.js \
 *     [--sheet-index 0] \
 *     [--batch-size 1000] \
 *     [--header true]
 *
 * Assumptions:
 * - First row is header when --header true (default). Columns become field names.
 * - Cells that look like JSON (starting with '{' or '[') are emitted as-is (unquoted) in the script.
 * - Blank cells emit null.
 * - Numeric and boolean types preserved; dates are emitted as ISO strings unless column header ends with ":date" to emit new Date().
 */
public class ExcelToMongo {

    public static void main(String[] args) throws Exception {
        Map<String, String> argMap = parseArgs(args);
        String inputPath = require(argMap, "--input");
        String dbName = require(argMap, "--db");
        String collection = require(argMap, "--collection");
        String outputPath = require(argMap, "--output");
        int sheetIndex = Integer.parseInt(argMap.getOrDefault("--sheet-index", "0"));
        int batchSize = Integer.parseInt(argMap.getOrDefault("--batch-size", "1000"));
        boolean hasHeader = Boolean.parseBoolean(argMap.getOrDefault("--header", "true"));

        List<Map<String, Object>> documents = new ArrayList<>();
        try (InputStream in = new FileInputStream(inputPath)) {
            Workbook workbook = createWorkbook(in, inputPath);
            Sheet sheet = workbook.getSheetAt(sheetIndex);
            List<String> headers = new ArrayList<>();
            int startRow = 0;
            if (hasHeader) {
                Row headerRow = sheet.getRow(0);
                if (headerRow == null) {
                    throw new IllegalArgumentException("Header row (0) is empty but --header true");
                }
                for (Cell cell : headerRow) {
                    headers.add(getCellString(cell));
                }
                startRow = 1;
            } else {
                // Generate generic headers col_0, col_1, ... based on first row cell count
                Row first = sheet.getRow(0);
                int cols = first != null ? first.getPhysicalNumberOfCells() : 0;
                for (int i = 0; i < cols; i++) headers.add("col_" + i);
            }

            DataFormatter formatter = new DataFormatter();

            for (int r = startRow; r <= sheet.getLastRowNum(); r++) {
                Row row = sheet.getRow(r);
                if (row == null) continue;
                Map<String, Object> doc = new LinkedHashMap<>();
                for (int c = 0; c < headers.size(); c++) {
                    String header = headers.get(c);
                    Cell cell = row.getCell(c, Row.MissingCellPolicy.RETURN_BLANK_AS_NULL);
                    Object value = cellToValue(cell, formatter, header);
                    doc.put(header, value);
                }
                if (!doc.isEmpty()) {
                    documents.add(doc);
                }
            }
        }

        String script = buildMongoScript(dbName, collection, documents, batchSize);
        try (Writer writer = new OutputStreamWriter(new FileOutputStream(outputPath), StandardCharsets.UTF_8)) {
            writer.write(script);
        }
        System.out.println("Generated: " + outputPath + " (" + documents.size() + " docs)");
    }

    private static Workbook createWorkbook(InputStream in, String path) throws IOException {
        if (path.toLowerCase(Locale.ROOT).endsWith(".xlsx")) {
            return new XSSFWorkbook(in);
        } else if (path.toLowerCase(Locale.ROOT).endsWith(".xls")) {
            return new HSSFWorkbook(in);
        }
        throw new IllegalArgumentException("Unsupported file type: " + path);
    }

    private static String require(Map<String, String> args, String key) {
        String v = args.get(key);
        if (v == null || v.trim().isEmpty()) {
            throw new IllegalArgumentException("Missing argument: " + key);
        }
        return v;
    }

    private static Map<String, String> parseArgs(String[] args) {
        Map<String, String> map = new HashMap<>();
        for (int i = 0; i < args.length; i++) {
            String a = args[i];
            if (a.startsWith("--")) {
                String next = (i + 1 < args.length && !args[i + 1].startsWith("--")) ? args[++i] : "true";
                map.put(a, next);
            }
        }
        return map;
    }

    private static String getCellString(Cell cell) {
        if (cell == null) return "";
        DataFormatter formatter = new DataFormatter();
        return formatter.formatCellValue(cell);
    }

    private static Object cellToValue(Cell cell, DataFormatter formatter, String header) {
        if (cell == null) return null;
        switch (cell.getCellType()) {
            case STRING:
                String s = cell.getStringCellValue();
                s = s != null ? s.trim() : null;
                if (s != null && looksLikeJson(s)) {
                    return new JsonLiteral(s);
                }
                return s == null || s.isEmpty() ? null : s;
            case NUMERIC:
                if (DateUtil.isCellDateFormatted(cell)) {
                    Date d = cell.getDateCellValue();
                    if (header != null && header.endsWith(":date")) {
                        return new JsonLiteral("new Date(\"" + iso(d) + "\")");
                    }
                    return iso(d);
                }
                double n = cell.getNumericCellValue();
                if (Math.floor(n) == n && n >= Long.MIN_VALUE && n <= Long.MAX_VALUE) {
                    return (long) n;
                }
                return n;
            case BOOLEAN:
                return cell.getBooleanCellValue();
            case FORMULA:
                try {
                    return cellToValue(evaluateFormula(cell), formatter, header);
                } catch (Exception e) {
                    String fv = formatter.formatCellValue(cell);
                    return fv == null || fv.isEmpty() ? null : fv;
                }
            case BLANK:
            default:
                String v = formatter.formatCellValue(cell);
                if (v != null && looksLikeJson(v)) return new JsonLiteral(v);
                return v == null || v.isEmpty() ? null : v;
        }
    }

    private static Cell evaluateFormula(Cell cell) {
        FormulaEvaluator evaluator = cell.getSheet().getWorkbook().getCreationHelper().createFormulaEvaluator();
        CellValue cv = evaluator.evaluate(cell);
        Cell tmp = cell.getRow().getSheet().getRow(cell.getRowIndex()).createCell(cell.getColumnIndex());
        switch (cv.getCellType()) {
            case STRING: tmp.setCellValue(cv.getStringValue()); break;
            case NUMERIC: tmp.setCellValue(cv.getNumberValue()); break;
            case BOOLEAN: tmp.setCellValue(cv.getBooleanValue()); break;
            default: tmp.setBlank();
        }
        return tmp;
    }

    private static boolean looksLikeJson(String s) {
        String t = s.trim();
        return (t.startsWith("{") && t.endsWith("}")) || (t.startsWith("[") && t.endsWith("]"));
    }

    private static String iso(Date d) {
        java.text.SimpleDateFormat f = new java.text.SimpleDateFormat("yyyy-MM-dd'T'HH:mm:ss.SSS'Z'");
        f.setTimeZone(java.util.TimeZone.getTimeZone("UTC"));
        return f.format(d);
    }

    private static String buildMongoScript(String dbName, String collection, List<Map<String, Object>> docs, int batchSize) {
        StringBuilder sb = new StringBuilder();
        sb.append("// Generated by ExcelToMongo\n");
        sb.append("db = db.getSiblingDB(\"").append(escapeJs(dbName)).append("\");\n");
        if (docs.isEmpty()) {
            sb.append("// No documents to insert\n");
            return sb.toString();
        }
        int count = 0;
        List<String> batch = new ArrayList<>();
        for (Map<String, Object> doc : docs) {
            batch.add(toJson(doc));
            if (batch.size() >= batchSize) {
                sb.append("db.").append(collection).append(".insertMany([").append(String.join(",", batch)).append("]);\n");
                batch.clear();
                count += batchSize;
            }
        }
        if (!batch.isEmpty()) {
            sb.append("db.").append(collection).append(".insertMany([").append(String.join(",", batch)).append("]);\n");
            count += batch.size();
        }
        sb.append("// Inserted ").append(count).append(" documents\n");
        return sb.toString();
    }

    private static String toJson(Map<String, Object> doc) {
        StringBuilder sb = new StringBuilder();
        sb.append("{");
        boolean first = true;
        for (Map.Entry<String, Object> e : doc.entrySet()) {
            if (!first) sb.append(",");
            first = false;
            sb.append("\"").append(escapeJs(e.getKey())).append("\":");
            sb.append(renderValue(e.getValue()));
        }
        sb.append("}");
        return sb.toString();
    }

    private static String renderValue(Object v) {
        if (v == null) return "null";
        if (v instanceof String) return "\"" + escapeJs((String) v) + "\"";
        if (v instanceof Number || v instanceof Boolean) return String.valueOf(v);
        if (v instanceof JsonLiteral) return ((JsonLiteral) v).literal;
        if (v instanceof Map) return toJson((Map<String, Object>) v);
        if (v instanceof List) {
            StringBuilder sb = new StringBuilder("[");
            boolean first = true;
            for (Object o : (List<?>) v) {
                if (!first) sb.append(",");
                first = false;
                sb.append(renderValue(o));
            }
            sb.append("]");
            return sb.toString();
        }
        return "\"" + escapeJs(String.valueOf(v)) + "\"";
    }

    private static String escapeJs(String s) {
        StringBuilder out = new StringBuilder();
        for (int i = 0; i < s.length(); i++) {
            char ch = s.charAt(i);
            switch (ch) {
                case '"': out.append("\\\""); break;
                case '\\': out.append("\\\\"); break;
                case '\n': out.append("\\n"); break;
                case '\r': out.append("\\r"); break;
                case '\t': out.append("\\t"); break;
                default:
                    if (ch < 32) {
                        out.append(String.format("\\u%04x", (int) ch));
                    } else {
                        out.append(ch);
                    }
            }
        }
        return out.toString();
    }

    private static final class JsonLiteral {
        final String literal;
        JsonLiteral(String literal) { this.literal = literal; }
    }
}
