# Term Mail

A lightweight, terminal-based email client built in Rust using the `ratatui` library for the UI and `imap`/`mailparse` crates to connect safely and securely to your IMAP provider over TLS.

## Features

- **TUI Interface**: A minimal, fast, and keyboard-driven terminal user interface powered by Ratatui.
- **IMAP Protocol**: Connects essentially to any modern mail provider to fetch real emails natively.
- **Split View Navigation**: Left pane for viewing the inbox queue securely and sorting, right pane for reading complete email contents via raw metadata body parsing.

## Setup

Before you begin, make sure to add your IMAP credentials to an environment file to authenticate to your mailbox.

1. **Create an Environment Variables file `(.env)`** inside the project root:

```sh
touch .env
```

2. **Add Your IMAP Details**:

```env
IMAP_SERVER=imap.example.com
IMAP_PORT=993
IMAP_USERNAME=your_email@example.com
IMAP_PASSWORD=your_app_password
```
*(Note for Gmail/Outlook/etc. users: Use an **App Password**; avoid typing out your raw account password!)*

## Usage

1. **To run the application**, execute Cargo inside the project directory:

```sh
cargo run
```

2. Look for the "Fetching latest emails over IMAP... Please wait." prompt. Once auth completes, it pulls down your 20 most recent messages, placing you securely back into the raw terminal mode interface!

3. **Navigation**: Use the `UP` \ `k` and `DOWN` \ `j` keys to change the currently highlighted email.
4. **Quitting**: Use `q` or `Esc` to safely quit back to your bash shell prompt. 

## Development & Testing

Unit tests run seamlessly via inherent native rust tooling validating cursor selection tracking and email navigation states natively.

```sh
cargo test
```
