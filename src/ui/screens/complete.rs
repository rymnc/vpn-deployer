use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::ServerInfo;
use crate::ui::centered_rect;

pub fn render(f: &mut Frame, area: Rect, server_info: &ServerInfo) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("🎉 Success! Your VPN is Ready")
        .title_alignment(Alignment::Center)
        .style(Style::default().fg(Color::Green));

    let complete_area = centered_rect(80, 70, area);
    f.render_widget(ratatui::widgets::Clear, complete_area);

    let inner = block.inner(complete_area);
    f.render_widget(block, complete_area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),
            Constraint::Length(8),
            Constraint::Min(0),
        ])
        .split(inner);

    // Server info
    let server_lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "Your VPN server is now running!",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("Server IP: ", Style::default().fg(Color::White)),
            Span::styled(
                &server_info.ip,
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("Server Name: ", Style::default().fg(Color::White)),
            Span::styled(
                &server_info.name,
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("Monthly Cost: ", Style::default().fg(Color::White)),
            Span::styled(
                &server_info.cost,
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ];

    let server_paragraph = Paragraph::new(server_lines)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(server_paragraph, chunks[0]);

    // Next steps
    let next_steps = vec![
        Line::from(Span::styled(
            "📱 Next Steps: Connect Your Devices",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("1. Install Tailscale on your devices:"),
        Line::from("   • Phone/Tablet: Get the app from your app store"),
        Line::from("   • Computer: Download from tailscale.com/download"),
        Line::from(""),
        Line::from("2. Sign in with the same Tailscale account"),
        Line::from("3. Your devices will automatically connect!"),
    ];

    let next_steps_paragraph = Paragraph::new(next_steps)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(next_steps_paragraph, chunks[1]);

    // Footer
    let footer_text = vec![
        Line::from(""),
        Line::from("🔒 Your VPN is secure and private"),
        Line::from("🌍 Access the internet from New York"),
        Line::from("💡 Manage your server at cloud.digitalocean.com"),
        Line::from(""),
        Line::from(Span::styled(
            "Press Enter to exit or 'q' to quit",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
    ];

    let footer_paragraph = Paragraph::new(footer_text)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(footer_paragraph, chunks[2]);
}
