use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use crate::app::{App, AppState};

pub mod components;
pub mod screens;

pub fn render(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0)].as_ref())
        .split(f.size());

    match &app.state {
        AppState::Welcome => screens::welcome::render(f, chunks[0]),
        AppState::Auth { token, cursor } => screens::auth::render(f, chunks[0], token, *cursor),
        AppState::RegionSelect { selected_index } => {
            screens::region_select::render(f, chunks[0], *selected_index)
        }
        AppState::TailscaleAuth { auth_key, cursor } => {
            screens::tailscale_auth::render(f, chunks[0], auth_key, *cursor)
        }
        AppState::Loading { message } => screens::loading::render(f, chunks[0], message),
        AppState::Deploy { progress } => {
            let region_name = app.selected_region.as_ref().map(|r| r.name.as_str());
            screens::deploy::render(f, chunks[0], progress, region_name);
        }
        AppState::Complete { server_info } => screens::complete::render(f, chunks[0], server_info),
        AppState::Error { message } => screens::error::render(f, chunks[0], message),
    }
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
