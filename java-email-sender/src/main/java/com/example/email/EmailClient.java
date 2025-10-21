package com.example.email;

import jakarta.mail.*;
import jakarta.mail.internet.InternetAddress;
import jakarta.mail.internet.MimeMessage;
import java.util.Properties;

public final class EmailClient {
    private final Session session;
    private final String fromAddress;

    private EmailClient(Session session, String fromAddress) {
        this.session = session;
        this.fromAddress = fromAddress;
    }

    public static EmailClient createSmtp(
            String host,
            int port,
            boolean useTls,
            String username,
            String password,
            String fromAddress
    ) {
        if (host == null || host.isBlank()) throw new IllegalArgumentException("host is required");
        if (fromAddress == null || fromAddress.isBlank()) throw new IllegalArgumentException("fromAddress is required");

        Properties props = new Properties();
        props.put("mail.smtp.auth", String.valueOf(username != null && !username.isBlank()));
        props.put("mail.smtp.starttls.enable", String.valueOf(useTls));
        props.put("mail.smtp.host", host);
        props.put("mail.smtp.port", String.valueOf(port));

        Authenticator authenticator = null;
        if (username != null && !username.isBlank()) {
            final String user = username;
            final String pass = password == null ? "" : password;
            authenticator = new Authenticator() {
                @Override
                protected PasswordAuthentication getPasswordAuthentication() {
                    return new PasswordAuthentication(user, pass);
                }
            };
        }

        Session session = Session.getInstance(props, authenticator);
        return new EmailClient(session, fromAddress);
    }

    public void send(String to, String subject, String body) {
        try {
            Message message = new MimeMessage(session);
            message.setFrom(new InternetAddress(fromAddress));
            message.setRecipients(Message.RecipientType.TO, InternetAddress.parse(to));
            message.setSubject(subject);
            message.setText(body);
            Transport.send(message);
        } catch (MessagingException e) {
            throw new RuntimeException("Failed to send email", e);
        }
    }
}
