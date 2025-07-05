use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::io;
use tokio::time::Duration;

mod app;
mod models;
mod services;
mod ui;

use app::{App, AppState};

fn print_help() {
    println!("VPN Deployer v{}", env!("CARGO_PKG_VERSION"));
    println!("Deploy your own VPN server on DigitalOcean with Tailscale");
    println!();
    println!("USAGE:");
    println!("    vpn-deployer [OPTIONS]");
    println!();
    println!("OPTIONS:");
    println!("    -h, --help     Print this help message");
    println!("    -v, --version  Print version information");
    println!();
    println!("DESCRIPTION:");
    println!("    This tool helps you deploy a VPN server on DigitalOcean using Tailscale.");
    println!("    You'll need:");
    println!("    • A DigitalOcean account and API token");
    println!("    • A Tailscale account and auth key");
    println!();
    println!("    The tool will guide you through the setup process interactively.");
    println!();
    println!("EXAMPLES:");
    println!("    vpn-deployer          Start the interactive setup");
    println!("    vpn-deployer --help   Show this help message");
    println!();
    println!("For more information, visit: https://github.com/rymnc/vpn-deployer");
}

#[tokio::main]
async fn main() -> Result<()> {
    // Handle command line arguments
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() > 1 {
        match args[1].as_str() {
            "--help" | "-h" => {
                print_help();
                return Ok(());
            }
            "--version" | "-v" => {
                println!("vpn-deployer {}", env!("CARGO_PKG_VERSION"));
                return Ok(());
            }
            _ => {
                eprintln!("Unknown argument: {}", args[1]);
                eprintln!("Use --help for usage information");
                std::process::exit(1);
            }
        }
    }

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {}", err);
    }

    Ok(())
}

async fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|f| ui::render(f, app))?;

        if let Ok(true) = event::poll(Duration::from_millis(100)) {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        if matches!(app.state, AppState::Welcome | AppState::Complete { .. } | AppState::Error { .. }) {
                            return Ok(());
                        }
                    }
                    KeyCode::Enter => {
                        app.handle_enter().await?;
                    }
                    KeyCode::Tab => {
                        app.handle_tab();
                    }
                    KeyCode::BackTab => {
                        app.handle_back_tab();
                    }
                    KeyCode::Char(c) => {
                        app.handle_char(c);
                    }
                    KeyCode::Backspace => {
                        app.handle_backspace();
                    }
                    _ => {}
                }
            }
        }

        // Handle async operations
        app.tick().await?;

        if app.should_quit {
            return Ok(());
        }
    }
}
