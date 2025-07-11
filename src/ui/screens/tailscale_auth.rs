use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
    Frame,
};

use crate::ui::centered_rect;

pub fn render(f: &mut Frame, area: Rect, auth_key: &str, _cursor: usize) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("🔑 Step 3: Tailscale Authentication")
        .title_alignment(Alignment::Center)
        .style(Style::default().fg(Color::Blue));

    let auth_area = centered_rect(80, 75, area);
    f.render_widget(Clear, auth_area);

    let inner = block.inner(auth_area);
    f.render_widget(block, auth_area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(12),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(inner);

    // Instructions
    let instructions = vec![
        Line::from(""),
        Line::from("Now we need a Tailscale auth key to connect your server."),
        Line::from(""),
        Line::from(Span::styled(
            "📋 Get your auth key at: https://login.tailscale.com/admin/settings/keys",
            Style::default().fg(Color::Cyan),
        )),
        Line::from(""),
        Line::from("Steps:"),
        Line::from("1. Click the link above (or visit it manually)"),
        Line::from("2. Log in to your Tailscale account"),
        Line::from("3. Click \"Generate auth key...\""),
        Line::from("4. Enable these options:"),
        Line::from(Span::styled(
            "   ✓ Reusable (REQUIRED - one-time keys will fail!)",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        )),
        Line::from("   ✓ Ephemeral (optional, for temporary servers)"),
        Line::from("5. Copy the auth key and paste it below"),
    ];

    let instructions_paragraph = Paragraph::new(instructions)
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: true });

    f.render_widget(instructions_paragraph, chunks[0]);

    // Auth key input
    let key_display = if auth_key.is_empty() {
        "Enter your Tailscale auth key...".to_string()
    } else {
        // Show first 12 chars and mask the rest for security
        if auth_key.len() > 12 {
            format!("{}{}", &auth_key[..12], "*".repeat(auth_key.len() - 12))
        } else {
            auth_key.to_string()
        }
    };

    let input_block = Block::default()
        .borders(Borders::ALL)
        .title("Tailscale Auth Key")
        .style(if auth_key.is_empty() {
            Style::default().fg(Color::Gray)
        } else {
            Style::default().fg(Color::Green)
        });

    let key_paragraph = Paragraph::new(key_display)
        .block(input_block)
        .style(Style::default().fg(Color::White));

    f.render_widget(key_paragraph, chunks[1]);

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
        Line::from("💡 Auth keys are used securely and not stored permanently."),
        Line::from("🔒 Your key will automatically configure the VPN server."),
        Line::from("🌐 The server will be set up as an exit node for secure browsing."),
        Line::from(""),
        Line::from(Span::styled(
            "⚠️  Common issues: Make sure the key is REUSABLE and not expired!",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
    ];

    let help_paragraph = Paragraph::new(help_text)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(help_paragraph, chunks[2]);
}
