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
        .title("❌ Error")
        .title_alignment(Alignment::Center)
        .style(Style::default().fg(Color::Red));

    let error_area = centered_rect(80, 50, area);
    f.render_widget(ratatui::widgets::Clear, error_area);

    let inner = block.inner(error_area);
    f.render_widget(block, error_area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(inner);

    // Error message
    let error_lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "Something went wrong:",
            Style::default()
                .fg(Color::Red)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            message,
            Style::default().fg(Color::White),
        )),
        Line::from(""),
        Line::from("Common issues:"),
        Line::from("• Invalid DigitalOcean API token"),
        Line::from("• Insufficient permissions on the token"),
        Line::from("• Network connection problems"),
        Line::from("• DigitalOcean service temporarily unavailable"),
        Line::from(""),
        Line::from("💡 Double-check your API token and try again."),
    ];

    let error_paragraph = Paragraph::new(error_lines)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(error_paragraph, chunks[0]);

    // Footer
    let footer_text = vec![
        Line::from(Span::styled(
            "Press Enter to go back or 'q' to quit",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
    ];

    let footer_paragraph = Paragraph::new(footer_text)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(footer_paragraph, chunks[1]);
}