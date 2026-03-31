use term_mail::app::{App, Email};

fn mock_emails() -> Vec<Email> {
    vec![
        Email {
            subject: "Subject 1".to_string(),
            sender: "sender1@test.com".to_string(),
            date: "Date 1".to_string(),
            body: "Body 1".to_string(),
            read: false,
        },
        Email {
            subject: "Subject 2".to_string(),
            sender: "sender2@test.com".to_string(),
            date: "Date 2".to_string(),
            body: "Body 2".to_string(),
            read: false,
        },
        Email {
            subject: "Subject 3".to_string(),
            sender: "sender3@test.com".to_string(),
            date: "Date 3".to_string(),
            body: "Body 3".to_string(),
            read: false,
        },
    ]
}

#[test]
fn test_app_new_empty() {
    let app = App::new(vec![]);
    assert!(app.emails.is_empty());
    assert_eq!(app.list_state.selected(), None);
}

#[test]
fn test_app_new_with_emails() {
    let app = App::new(mock_emails());
    assert_eq!(app.emails.len(), 3);
    assert_eq!(app.list_state.selected(), Some(0));
    assert!(app.emails[0].read);
}

#[test]
fn test_app_next() {
    let mut app = App::new(mock_emails());
    app.next();
    assert_eq!(app.list_state.selected(), Some(1));
    assert!(app.emails[1].read);
    app.next();
    assert_eq!(app.list_state.selected(), Some(2));
    assert!(app.emails[2].read);
    // Test bounds
    app.next();
    assert_eq!(app.list_state.selected(), Some(2));
}

#[test]
fn test_app_previous() {
    let mut app = App::new(mock_emails());
    app.next();
    app.next();

    app.previous();
    assert_eq!(app.list_state.selected(), Some(1));
    app.previous();
    assert_eq!(app.list_state.selected(), Some(0));
    // Test bounds
    app.previous();
    assert_eq!(app.list_state.selected(), Some(0));
}
