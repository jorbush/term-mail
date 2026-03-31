use crate::app::Email;
use std::env;

pub fn fetch_emails() -> Result<Vec<Email>, Box<dyn std::error::Error>> {
    let domain = env::var("IMAP_SERVER")?;
    let username = env::var("IMAP_USERNAME")?;
    let password = env::var("IMAP_PASSWORD")?;

    // Default to port 993 (IMAPS) if not provided
    let port: u16 = env::var("IMAP_PORT")
        .unwrap_or_else(|_| "993".to_string())
        .parse()
        .unwrap_or(993);

    let tls = native_tls::TlsConnector::builder().build()?;

    // Connect to the IMAP server
    let client = imap::connect((domain.as_str(), port), domain.as_str(), &tls)?;

    // Login
    let mut session = client.login(username, password).map_err(|e| e.0)?;

    // Select INBOX
    session.select("INBOX")?;

    // Fetch messages. We'll fetch the last 20 messages.
    let messages = session.search("ALL")?;
    let mut message_ids: Vec<u32> = messages.into_iter().collect();
    message_ids.sort_unstable(); // Ensure they are ordered

    // Take the last 20 message IDs
    let latest_ids: Vec<u32> = message_ids.into_iter().rev().take(20).collect();

    let mut emails = Vec::new();

    if !latest_ids.is_empty() {
        // Construct the sequence set (e.g., "1,2,3")
        let sequence_set = latest_ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let fetched_messages = session.fetch(sequence_set, "(RFC822)")?;

        for msg in fetched_messages.iter() {
            let body_bytes = if let Some(body) = msg.body() {
                body
            } else {
                continue;
            };

            let parsed_mail = match mailparse::parse_mail(body_bytes) {
                Ok(mail) => mail,
                Err(_) => continue,
            };

            let mut subject = String::new();
            let mut sender = String::new();
            let mut date = String::new();

            for header in &parsed_mail.headers {
                match header.get_key().to_lowercase().as_str() {
                    "subject" => subject = header.get_value(),
                    "from" => sender = header.get_value(),
                    "date" => date = header.get_value(),
                    _ => {}
                }
            }

            let body_text = parsed_mail.get_body().unwrap_or_default();

            emails.push(Email {
                subject,
                sender,
                date,
                body: body_text,
                read: true, // simplified for now
            });
        }
    }

    // Logout
    session.logout()?;

    // imap fetch doesn't guarantee order, so we reverse it or just let it be.
    // Let's reverse it to show newest first.
    emails.reverse();

    Ok(emails)
}
