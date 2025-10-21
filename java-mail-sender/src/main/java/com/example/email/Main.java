package com.example.email;

import jakarta.mail.Authenticator;
import jakarta.mail.Message;
import jakarta.mail.MessagingException;
import jakarta.mail.PasswordAuthentication;
import jakarta.mail.Session;
import jakarta.mail.Transport;
import jakarta.mail.internet.InternetAddress;
import jakarta.mail.internet.MimeMessage;

import java.util.Properties;

public final class Main {

    public static void main(String[] args) throws Exception {
        String smtpHost = getenvOrThrow("SMTP_HOST");
        String smtpPort = getenvDefault("SMTP_PORT", "587");
        String smtpUser = getenvOrThrow("SMTP_USER");
        String smtpPass = getenvOrThrow("SMTP_PASS");
        String smtpFrom = getenvOrThrow("SMTP_FROM");
        String smtpTo = getenvOrThrow("SMTP_TO");

        String user = getenvOrThrow("TPL_USER");
        String product = getenvOrThrow("TPL_PRODUCT");
        String action1 = getenvOrThrow("TPL_ACTION1");
        String action2 = getenvOrThrow("TPL_ACTION2");

        String subject = System.getenv().getOrDefault("EMAIL_SUBJECT", "Welcome to " + product);

        String html = HtmlTemplateRenderer.renderWelcomeEmail(user, product, action1, action2);

        sendEmail(smtpHost, smtpPort, smtpUser, smtpPass, smtpFrom, smtpTo, subject, html);
        System.out.println("Email sent to " + smtpTo);
    }

    private static void sendEmail(
            String host,
            String port,
            String username,
            String password,
            String from,
            String to,
            String subject,
            String html
    ) throws MessagingException {
        Properties props = new Properties();
        props.put("mail.smtp.auth", "true");
        props.put("mail.smtp.starttls.enable", "true");
        props.put("mail.smtp.host", host);
        props.put("mail.smtp.port", port);

        Session session = Session.getInstance(props, new Authenticator() {
            @Override
            protected PasswordAuthentication getPasswordAuthentication() {
                return new PasswordAuthentication(username, password);
            }
        });

        MimeMessage message = new MimeMessage(session);
        message.setFrom(new InternetAddress(from));
        message.setRecipients(Message.RecipientType.TO, InternetAddress.parse(to));
        message.setSubject(subject, "UTF-8");
        message.setContent(html, "text/html; charset=UTF-8");

        Transport.send(message);
    }

    private static String getenvOrThrow(String key) {
        String value = System.getenv(key);
        if (value == null || value.isBlank()) {
            throw new IllegalArgumentException("Missing required env var: " + key);
        }
        return value;
    }

    private static String getenvDefault(String key, String defaultValue) {
        String value = System.getenv(key);
        return (value == null || value.isBlank()) ? defaultValue : value;
    }
}
