use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

use crate::ui::centered_rect;

pub fn render(f: &mut Frame, area: Rect, message: &str) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("🚀 Preparing Your VPN Server")
        .title_alignment(Alignment::Center)
        .style(Style::default().fg(Color::Blue));

    let loading_area = centered_rect(60, 40, area);
    f.render_widget(ratatui::widgets::Clear, loading_area);

    let inner = block.inner(loading_area);
    f.render_widget(block, loading_area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(inner);

    // Loading spinner/message
    let loading_lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            message,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
    ];

    let loading_paragraph = Paragraph::new(loading_lines)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(loading_paragraph, chunks[0]);

    // Spinner animation
    let spinner_lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
    ];

    let spinner_paragraph = Paragraph::new(spinner_lines)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(spinner_paragraph, chunks[1]);

    // Info text
    let info_text = vec![
        Line::from("Please wait while we set up your server..."),
        Line::from(""),
        Line::from("This process may take a few minutes."),
    ];

    let info_paragraph = Paragraph::new(info_text)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(info_paragraph, chunks[2]);
}
