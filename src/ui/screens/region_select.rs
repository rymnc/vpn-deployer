use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::models::RegionOption;
use crate::ui::centered_rect;

pub fn render(f: &mut Frame, area: Rect, selected_index: usize) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("🌍 Step 2: Select Server Location")
        .title_alignment(Alignment::Center)
        .style(Style::default().fg(Color::Blue));

    let select_area = centered_rect(70, 70, area);
    f.render_widget(ratatui::widgets::Clear, select_area);

    let inner = block.inner(select_area);
    f.render_widget(block, select_area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(4),
        ])
        .split(inner);

    // Instructions
    let instructions = vec![
        Line::from("Use ↑/↓ arrows (or W/S/J/K) to select a region"),
        Line::from("Press Enter to confirm your selection"),
    ];

    let instructions_paragraph = Paragraph::new(instructions)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(instructions_paragraph, chunks[0]);

    // Region list
    let regions = RegionOption::available_regions();
    let items: Vec<ListItem> = regions
        .iter()
        .enumerate()
        .map(|(i, region)| {
            let style = if i == selected_index {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            let prefix = if i == selected_index { "► " } else { "  " };
            
            let content = vec![
                Line::from(vec![
                    Span::styled(prefix, style),
                    Span::styled(&region.name, style),
                ]),
                Line::from(vec![
                    Span::styled("    ", style),
                    Span::styled(&region.description, Style::default().fg(Color::Gray)),
                ]),
                Line::from(""),
            ];

            ListItem::new(content).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Available Regions ($4/month)")
            .title_alignment(Alignment::Center)
            .style(Style::default().fg(Color::Green)));

    f.render_widget(list, chunks[1]);

    // Footer info
    let footer = vec![
        Line::from("💰 All regions shown offer $4/month pricing"),
        Line::from("📍 Choose the region closest to you for best performance"),
        Line::from("🔒 Your VPN server will be deployed in the selected region"),
    ];

    let footer_paragraph = Paragraph::new(footer)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(footer_paragraph, chunks[2]);
}