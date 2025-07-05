use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
    Frame,
};

use crate::ui::centered_rect;

pub fn render(f: &mut Frame, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("🚀 Tailscale VPN Deployer")
        .title_alignment(Alignment::Center)
        .style(Style::default().fg(Color::Blue));

    let welcome_area = centered_rect(80, 60, area);
    f.render_widget(Clear, welcome_area);

    let inner = block.inner(welcome_area);
    f.render_widget(block, welcome_area);

    let content = vec![
        Line::from(""),
        Line::from(Span::styled(
            "Welcome to Tailscale VPN Deployer!",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("This tool will create a VPN server for you on DigitalOcean."),
        Line::from("No technical knowledge required - just follow the steps!"),
        Line::from(""),
        Line::from("What we'll do:"),
        Line::from("• Create a $4/month server on DigitalOcean"),
        Line::from("• Install and configure Tailscale VPN"),
        Line::from("• Connect your devices to the VPN"),
        Line::from(""),
        Line::from("Requirements:"),
        Line::from("• DigitalOcean account with API token"),
        Line::from("• Tailscale account (free)"),
        Line::from(""),
        Line::from(""),
        Line::from(Span::styled(
            "Press Enter to continue or 'q' to quit",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
    ];

    let paragraph = Paragraph::new(content)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, inner);
}