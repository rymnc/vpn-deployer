use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Gauge, Paragraph, Wrap},
    Frame,
};

use crate::app::DeployProgress;
use crate::ui::centered_rect;

pub fn render(f: &mut Frame, area: Rect, progress: &DeployProgress, region_name: Option<&str>) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("🚀 Step 4: Deploying Your VPN Server")
        .title_alignment(Alignment::Center)
        .style(Style::default().fg(Color::Blue));

    let deploy_area = centered_rect(80, 60, area);
    f.render_widget(ratatui::widgets::Clear, deploy_area);

    let inner = block.inner(deploy_area);
    f.render_widget(block, deploy_area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),
            Constraint::Length(3),
            Constraint::Length(5),
            Constraint::Min(0),
        ])
        .split(inner);

    // Step indicators
    let steps = vec![
        ("✓", "Validating credentials", progress.current_step > 1),
        ("⏳", "Creating server ($4/month)", progress.current_step > 2),
        ("⏳", "Waiting for server to be ready", progress.current_step > 3),
        ("⏳", "Installing and configuring Tailscale", progress.current_step > 4),
        ("⏳", "Connecting to your Tailnet", progress.current_step > 5),
    ];

    let step_lines: Vec<Line> = steps
        .iter()
        .enumerate()
        .map(|(i, (_icon, text, completed))| {
            let step_num = i + 1;
            let icon_char = if *completed {
                "✓"
            } else if step_num == progress.current_step {
                "⏳"
            } else {
                "⏳"
            };

            let style = if *completed {
                Style::default().fg(Color::Green)
            } else if step_num == progress.current_step {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default().fg(Color::Gray)
            };

            Line::from(vec![
                Span::styled(format!("{} ", icon_char), style),
                Span::styled(text.to_string(), style),
            ])
        })
        .collect();

    let steps_paragraph = Paragraph::new(step_lines)
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: true });

    f.render_widget(steps_paragraph, chunks[0]);

    // Progress bar
    let progress_ratio = progress.current_step as f64 / progress.total_steps as f64;
    let progress_bar = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Progress"))
        .gauge_style(Style::default().fg(Color::Green))
        .percent((progress_ratio * 100.0) as u16);

    f.render_widget(progress_bar, chunks[1]);

    // Current status
    let status_lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            &progress.status,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
    ];

    let status_paragraph = Paragraph::new(status_lines)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(status_paragraph, chunks[2]);

    // Info text
    let region_display = region_name.unwrap_or("New York");
    let info_text = vec![
        Line::from("This usually takes 2-3 minutes..."),
        Line::from(""),
        Line::from("💰 Server cost: $4/month (~$0.006/hour)"),
        Line::from(format!("📍 Server location: {}", region_display)),
        Line::from("💾 Server specs: 512MB RAM, 1 CPU, 10GB SSD"),
    ];

    let info_paragraph = Paragraph::new(info_text)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(info_paragraph, chunks[3]);
}