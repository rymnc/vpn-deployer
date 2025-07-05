use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

use crate::ui::centered_rect;

pub fn render(f: &mut Frame, area: Rect, token: &str, _cursor: usize) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("🔐 Step 1: Authentication")
        .title_alignment(Alignment::Center)
        .style(Style::default().fg(Color::Blue));

    let auth_area = centered_rect(80, 70, area);
    f.render_widget(ratatui::widgets::Clear, auth_area);

    let inner = block.inner(auth_area);
    f.render_widget(block, auth_area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(10),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(inner);

    // Instructions
    let instructions = vec![
        Line::from(""),
        Line::from("We need your DigitalOcean API token to create a server."),
        Line::from(""),
        Line::from(Span::styled(
            "📋 Get your token at: https://cloud.digitalocean.com/account/api",
            Style::default().fg(Color::Cyan),
        )),
        Line::from(""),
        Line::from("Steps:"),
        Line::from("1. Log in to your DigitalOcean account"),
        Line::from("2. Go to API → Personal Access Tokens"),
        Line::from("3. Generate New Token with read/write permissions"),
        Line::from("4. Copy and paste the token below"),
    ];

    let instructions_paragraph = Paragraph::new(instructions)
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: true });

    f.render_widget(instructions_paragraph, chunks[0]);

    // Token input
    let token_display = if token.is_empty() {
        "Enter your DigitalOcean API token...".to_string()
    } else {
        // Show first 8 chars and mask the rest
        if token.len() > 8 {
            format!("{}{}", &token[..8], "*".repeat(token.len() - 8))
        } else {
            token.to_string()
        }
    };

    let input_block = Block::default()
        .borders(Borders::ALL)
        .title("API Token")
        .style(if token.is_empty() {
            Style::default().fg(Color::Gray)
        } else {
            Style::default().fg(Color::Green)
        });

    let token_paragraph = Paragraph::new(token_display)
        .block(input_block)
        .style(Style::default().fg(Color::White));

    f.render_widget(token_paragraph, chunks[1]);

    // Help text
    let help_text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "Press Enter to continue or 'q' to quit",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("💡 Your token will be used securely and not stored permanently."),
    ];

    let help_paragraph = Paragraph::new(help_text)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(help_paragraph, chunks[2]);
}