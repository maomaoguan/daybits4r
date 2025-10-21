package com.example.email;

public final class HtmlTemplateRenderer {

    private HtmlTemplateRenderer() {}

    public static String renderWelcomeEmail(
            String user,
            String productOrOrg,
            String actionOne,
            String actionTwo
    ) {
        String safeUser = escapeHtml(user);
        String safeProduct = escapeHtml(productOrOrg);
        String safeAction1 = escapeHtml(actionOne);
        String safeAction2 = escapeHtml(actionTwo);

        String indent = "    ";
        StringBuilder sb = new StringBuilder(512);
        sb.append("<!DOCTYPE html>\n");
        sb.append("<html lang=\"en\">\n");
        sb.append("<head>\n");
        sb.append(indent).append("<meta charset=\"UTF-8\">\n");
        sb.append(indent).append("<meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">\n");
        sb.append(indent).append("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
        sb.append(indent).append("<title>Welcome</title>\n");
        sb.append("</head>\n");
        sb.append("<body style=\"font-family: -apple-system, BlinkMacSystemFont, Segoe UI, Roboto, Helvetica, Arial, sans-serif; color: #111; line-height: 1.5;\">\n");
        sb.append(indent).append("<div>\n");
        sb.append(indent).append(indent).append("<p>dear ").append(safeUser).append(", welcome to ").append(safeProduct).append(",</p>\n");
        sb.append(indent).append(indent).append("<p>there are a few actions to do,</p>\n");
        sb.append(indent).append(indent).append("<p>1: ").append(safeAction1).append(";</p>\n");
        sb.append(indent).append(indent).append("<p>2: ").append(safeAction2).append(";</p>\n");
        sb.append(indent).append(indent).append("<p>thanks for your corporation.</p>\n");
        sb.append(indent).append("</div>\n");
        sb.append("</body>\n");
        sb.append("</html>\n");
        return sb.toString();
    }

    private static String escapeHtml(String input) {
        if (input == null) return "";
        StringBuilder out = new StringBuilder(Math.max(16, input.length()));
        for (int i = 0; i < input.length(); i++) {
            char c = input.charAt(i);
            switch (c) {
                case '&': out.append("&amp;"); break;
                case '<': out.append("&lt;"); break;
                case '>': out.append("&gt;"); break;
                case '\"': out.append("&quot;"); break;
                case '\'': out.append("&#x27;"); break;
                default: out.append(c);
            }
        }
        return out.toString();
    }
}
