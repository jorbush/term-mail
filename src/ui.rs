use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
};

use crate::app::App;

pub fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.area());

    // Title / Header
    let title = Paragraph::new(Line::from(vec![
        Span::styled(
            " T",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("erm"),
        Span::styled(
            "M",
            Style::default()
                .fg(Color::LightBlue)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("ail - Inbox"),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray)),
    );
    f.render_widget(title, chunks[0]);

    // Main Content Area (Split Left/Right)
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(chunks[1]);

    draw_email_list(f, app, main_chunks[0]);
    draw_email_content(f, app, main_chunks[1]);
}

fn draw_email_list(f: &mut Frame, app: &mut App, area: Rect) {
    let items: Vec<ListItem> = app
        .emails
        .iter()
        .map(|e| {
            let mut style = Style::default();
            if !e.read {
                style = style.add_modifier(Modifier::BOLD).fg(Color::Cyan);
            }

            let content = vec![
                Line::from(Span::styled(e.sender.clone(), style)),
                Line::from(Span::raw(e.subject.clone())),
                Line::from(Span::styled(
                    e.date.clone(),
                    Style::default().fg(Color::DarkGray),
                )),
            ];
            ListItem::new(content)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().title(" Inbox ").borders(Borders::ALL))
        .highlight_style(
            Style::default()
                .bg(Color::Rgb(40, 44, 52))
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(list, area, &mut app.list_state);
}

fn draw_email_content(f: &mut Frame, app: &App, area: Rect) {
    if let Some(selected) = app.list_state.selected()
        && let Some(email) = app.emails.get(selected)
    {
        let content = vec![
            Line::from(vec![
                Span::styled("From: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(&email.sender),
            ]),
            Line::from(vec![
                Span::styled("Date: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(&email.date),
            ]),
            Line::from(vec![
                Span::styled("Subject: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(&email.subject),
            ]),
            Line::from(""),
            Line::from(Span::raw(&email.body)),
        ];

        let p = Paragraph::new(content)
            .block(Block::default().title(" Message ").borders(Borders::ALL))
            .wrap(Wrap { trim: true });

        f.render_widget(p, area);
        return;
    }

    // Default view if no email is selected
    let p = Paragraph::new("Select an email from the inbox to read it.")
        .block(Block::default().title(" Message ").borders(Borders::ALL));
    f.render_widget(p, area);
}
