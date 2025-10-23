package com.example;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import org.apache.poi.ss.usermodel.*;
import org.apache.poi.xssf.usermodel.XSSFWorkbook;

import java.io.*;
import java.nio.file.Files;
import java.nio.file.Path;
import java.time.format.DateTimeFormatter;
import java.util.*;

public class ExcelToMongoScript {
    private static final ObjectMapper MAPPER = new ObjectMapper();

    public static void main(String[] args) throws Exception {
        Args cfg = Args.parse(args);
        if (cfg == null) {
            printUsage();
            System.exit(1);
            return;
        }
        List<Map<String, Object>> documents = readExcel(cfg.inputPath, cfg.sheetName, cfg.headerRowIndex, cfg.dataStartRowIndex);
        String script = buildMongoshScript(cfg.databaseName, cfg.collectionName, documents, cfg.insertBatchSize);
        Path out = Path.of(cfg.outputPath);
        Files.createDirectories(out.getParent());
        Files.writeString(out, script);
        System.out.println("Generated script: " + out.toAbsolutePath());
    }

    private static void printUsage() {
        System.out.println("Usage: java -jar excel-to-mongo.jar --input <xlsx> --db <dbName> --collection <collectionName> [--sheet <sheetName>] [--out <script.js>] [--headerRow 0] [--dataStartRow 1] [--batch 1000]");
    }

    static List<Map<String, Object>> readExcel(String inputPath, String sheetName, int headerRowIndex, int dataStartRowIndex) throws IOException {
        try (InputStream in = new FileInputStream(inputPath); Workbook workbook = new XSSFWorkbook(in)) {
            Sheet sheet = sheetName == null ? workbook.getSheetAt(0) : workbook.getSheet(sheetName);
            if (sheet == null) throw new IllegalArgumentException("Sheet not found: " + sheetName);

            Row headerRow = sheet.getRow(headerRowIndex);
            if (headerRow == null) throw new IllegalArgumentException("Header row not found at index " + headerRowIndex);
            List<String> headers = new ArrayList<>();
            for (Cell cell : headerRow) {
                headers.add(getCellString(cell));
            }

            List<Map<String, Object>> documents = new ArrayList<>();
            for (int r = dataStartRowIndex; r <= sheet.getLastRowNum(); r++) {
                Row row = sheet.getRow(r);
                if (row == null) continue; // skip empty
                Map<String, Object> doc = new LinkedHashMap<>();
                boolean allBlank = true;
                for (int c = 0; c < headers.size(); c++) {
                    String key = headers.get(c);
                    if (key == null || key.isBlank()) continue;
                    Cell cell = row.getCell(c, Row.MissingCellPolicy.RETURN_BLANK_AS_NULL);
                    Object value = getCellValue(cell);
                    if (value != null && !(value instanceof String && ((String) value).isBlank())) {
                        allBlank = false;
                    }
                    if (value != null) {
                        // Attempt to parse JSON for string-looking values
                        Object maybeJson = tryParseJson(value);
                        doc.put(key, maybeJson);
                    }
                }
                if (!doc.isEmpty() && !allBlank) {
                    documents.add(doc);
                }
            }
            return documents;
        }
    }

    private static Object tryParseJson(Object value) {
        if (!(value instanceof String str)) return value;
        String s = str.trim();
        if (s.isEmpty()) return null;
        // Heuristic: likely JSON if starts with { [ " digit true false null
        char ch = s.charAt(0);
        if (ch == '{' || ch == '[' || ch == '"' || ch == '-' || Character.isDigit(ch) || s.equals("true") || s.equals("false") || s.equals("null")) {
            try {
                JsonNode node = MAPPER.readTree(s);
                return MAPPER.convertValue(node, Object.class);
            } catch (Exception ignore) {
                // fallthrough to plain string
            }
        }
        return s;
    }

    private static String getCellString(Cell cell) {
        if (cell == null) return null;
        return switch (cell.getCellType()) {
            case STRING -> cell.getStringCellValue();
            case NUMERIC -> {
                if (DateUtil.isCellDateFormatted(cell)) {
                    Date date = cell.getDateCellValue();
                    yield isoDate(date);
                } else {
                    double d = cell.getNumericCellValue();
                    if (d == Math.rint(d)) {
                        yield String.valueOf((long) d);
                    }
                    yield String.valueOf(d);
                }
            }
            case BOOLEAN -> String.valueOf(cell.getBooleanCellValue());
            case FORMULA -> {
                try {
                    FormulaEvaluator evaluator = cell.getSheet().getWorkbook().getCreationHelper().createFormulaEvaluator();
                    CellValue cv = evaluator.evaluate(cell);
                    if (cv == null) yield null;
                    yield switch (cv.getCellType()) {
                        case STRING -> cv.getStringValue();
                        case NUMERIC -> {
                            if (DateUtil.isCellDateFormatted(cell)) {
                                Date date = cell.getDateCellValue();
                                yield isoDate(date);
                            } else {
                                double d = cv.getNumberValue();
                                if (d == Math.rint(d)) {
                                    yield String.valueOf((long) d);
                                }
                                yield String.valueOf(d);
                            }
                        }
                        case BOOLEAN -> String.valueOf(cv.getBooleanValue());
                        default -> null;
                    };
                } catch (Exception e) {
                    yield cell.getCellFormula();
                }
            }
            default -> null;
        };
    }

    private static Object getCellValue(Cell cell) {
        if (cell == null) return null;
        return switch (cell.getCellType()) {
            case STRING -> cell.getStringCellValue();
            case NUMERIC -> {
                if (DateUtil.isCellDateFormatted(cell)) {
                    Date date = cell.getDateCellValue();
                    yield Map.of("$date", isoDate(date));
                } else {
                    double d = cell.getNumericCellValue();
                    if (d == Math.rint(d)) {
                        yield (long) d;
                    }
                    yield d;
                }
            }
            case BOOLEAN -> cell.getBooleanCellValue();
            case FORMULA -> {
                try {
                    FormulaEvaluator evaluator = cell.getSheet().getWorkbook().getCreationHelper().createFormulaEvaluator();
                    CellValue cv = evaluator.evaluate(cell);
                    if (cv == null) yield null;
                    yield switch (cv.getCellType()) {
                        case STRING -> cv.getStringValue();
                        case NUMERIC -> {
                            if (DateUtil.isCellDateFormatted(cell)) {
                                Date date = cell.getDateCellValue();
                                yield Map.of("$date", isoDate(date));
                            } else {
                                double d = cv.getNumberValue();
                                if (d == Math.rint(d)) {
                                    yield (long) d;
                                }
                                yield d;
                            }
                        }
                        case BOOLEAN -> cv.getBooleanValue();
                        default -> null;
                    };
                } catch (Exception e) {
                    yield cell.getCellFormula();
                }
            }
            default -> null;
        };
    }

    private static String isoDate(Date date) {
        return new java.text.SimpleDateFormat("yyyy-MM-dd'T'HH:mm:ss.SSSX").format(date);
    }

    static String buildMongoshScript(String dbName, String collection, List<Map<String, Object>> docs, int batchSize) throws JsonProcessingException {
        StringBuilder sb = new StringBuilder();
        sb.append("// Generated by ExcelToMongoScript\n");
        sb.append("const database = db.getSiblingDB(\"")
          .append(escapeJsString(dbName))
          .append("\");\n");
        sb.append("const coll = database.getCollection(\"")
          .append(escapeJsString(collection))
          .append("\");\n");
        sb.append("// coll.drop(); // optional: clear collection before insert\n");

        int i = 0;
        while (i < docs.size()) {
            int end = Math.min(i + batchSize, docs.size());
            List<Map<String, Object>> batch = docs.subList(i, end);
            String jsonArray = MAPPER.writer().writeValueAsString(batch);
            String jsonEscaped = escapeForSingleQuotedJsString(jsonArray);
            sb.append("const docs = EJSON.parse('")
              .append(jsonEscaped)
              .append("');\n");
            sb.append("coll.insertMany(docs);\n");
            i = end;
        }
        return sb.toString();
    }

    private static String escapeJsString(String input) {
        return input.replace("\\", "\\\\").replace("\"", "\\\"").replace("`", "\\`");
    }

    private static String escapeForSingleQuotedJsString(String input) {
        if (input == null) return "";
        StringBuilder out = new StringBuilder(input.length() + 16);
        for (int i = 0; i < input.length(); i++) {
            char ch = input.charAt(i);
            switch (ch) {
                case '\\' -> out.append("\\\\");
                case '\'' -> out.append("\\'");
                case '\n' -> out.append("\\n");
                case '\r' -> out.append("\\r");
                case '\t' -> out.append("\\t");
                case '\u2028' -> out.append("\\u2028");
                case '\u2029' -> out.append("\\u2029");
                default -> out.append(ch);
            }
        }
        return out.toString();
    }

    private record Args(String inputPath, String databaseName, String collectionName, String sheetName,
                        String outputPath, int headerRowIndex, int dataStartRowIndex, int insertBatchSize) {
        static Args parse(String[] args) {
            Map<String, String> m = new HashMap<>();
            for (int i = 0; i < args.length - 1; i += 2) {
                String k = args[i];
                String v = args[i + 1];
                if (k.startsWith("--")) m.put(k.substring(2), v);
            }
            if (!m.containsKey("input") || !m.containsKey("db") || !m.containsKey("collection")) return null;
            String sheet = m.getOrDefault("sheet", null);
            String out = m.getOrDefault("out", "./mongo_insert.js");
            int headerRow = Integer.parseInt(m.getOrDefault("headerRow", "0"));
            int dataStart = Integer.parseInt(m.getOrDefault("dataStartRow", "1"));
            int batch = Integer.parseInt(m.getOrDefault("batch", "1000"));
            return new Args(m.get("input"), m.get("db"), m.get("collection"), sheet, out, headerRow, dataStart, batch);
        }
    }
}
