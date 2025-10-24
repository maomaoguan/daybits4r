package com.example.excel2mongo;

import java.io.IOException;

public final class Main {
    private Main() {}

    public static void main(String[] args) {
        if (args.length != 1) {
            System.err.println("Usage: java -jar excel2mongo.jar <path-to-excel>");
            System.exit(1);
        }
        String path = args[0];
        try {
            String script = ExcelToMongoScriptGenerator.generateMongoInsertScript(path);
            System.out.println(script);
        } catch (IOException e) {
            System.err.println("Error: " + e.getMessage());
            System.exit(2);
        }
    }
}
