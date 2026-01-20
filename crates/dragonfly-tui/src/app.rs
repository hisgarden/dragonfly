//! Main TUI application
//!
//! This module provides the full-screen terminal UI with defrag animation.

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::{
    io,
    time::{Duration, Instant},
};

use crate::animation::DefragAnimation;

/// Application state
pub struct App {
    /// Should the app quit?
    pub should_quit: bool,
    /// Defrag animation
    animation: DefragAnimation,
    /// Scan progress (0.0 to 1.0)
    progress: f64,
    /// Total bytes scanned
    bytes_scanned: u64,
    /// Total files scanned
    files_scanned: u64,
    /// Target path being scanned
    target_path: String,
}

impl App {
    /// Create a new app
    pub fn new(target_path: String) -> Self {
        Self {
            should_quit: false,
            animation: DefragAnimation::default_size(),
            progress: 0.0,
            bytes_scanned: 0,
            files_scanned: 0,
            target_path,
        }
    }
    
    /// Update the app state
    pub fn update(&mut self) {
        // Update animation
        self.animation.update();
        
        // Simulate scan progress (this will be real data from dragonfly-disk later)
        if self.progress < 1.0 {
            self.progress += 0.001;
            self.bytes_scanned += 1024 * 1024; // 1MB per frame
            self.files_scanned += 10;
        }
    }
    
    /// Handle input events
    pub fn handle_event(&mut self, event: Event) -> Result<()> {
        if let Event::Key(key) = event {
            self.handle_key_event(key)?;
        }
        Ok(())
    }
    
    /// Handle key events
    fn handle_key_event(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                self.should_quit = true;
            }
            KeyCode::Char('c') if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                self.should_quit = true;
            }
            _ => {}
        }
        Ok(())
    }
    
    /// Draw the UI
    pub fn draw(&mut self, frame: &mut Frame) {
        // Create layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),   // Title
                Constraint::Min(20),     // Animation area
                Constraint::Length(5),   // Progress/stats
                Constraint::Length(3),   // Help
            ])
            .split(frame.size());
        
        // Title
        let title = Paragraph::new("🐉 DragonFly Defrag Theater")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(title, chunks[0]);
        
        // Animation area
        let animation_text = self.animation.render();
        let animation = Paragraph::new(animation_text)
            .style(Style::default().fg(Color::Green))
            .block(Block::default().borders(Borders::ALL).title("Disk Allocation"));
        frame.render_widget(animation, chunks[1]);
        
        // Progress/stats
        let progress_pct = (self.progress * 100.0) as u32;
        let bytes_gb = self.bytes_scanned as f64 / (1024.0 * 1024.0 * 1024.0);
        let files_k = self.files_scanned / 1000;
        
        let progress_bar = "█".repeat(progress_pct.min(100) as usize / 2);
        let progress_text = format!(
            "Scanning {} … {}% | {:.1} GB indexed | {}K files\n{}",
            self.target_path,
            progress_pct,
            bytes_gb,
            files_k,
            progress_bar
        );
        
        let progress = Paragraph::new(progress_text)
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title("Progress"));
        frame.render_widget(progress, chunks[2]);
        
        // Help text
        let help = Paragraph::new(vec![
            Line::from(vec![
                Span::styled("Q", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::raw(" = Quit  "),
                Span::styled("Ctrl+C", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::raw(" = Exit"),
            ]),
        ])
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
        frame.render_widget(help, chunks[3]);
    }
}

/// Run the TUI application
pub async fn run_app(target_path: String) -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    // Create app state
    let mut app = App::new(target_path);
    
    // Event loop
    let tick_rate = Duration::from_millis(100);
    let mut last_tick = Instant::now();
    
    loop {
        // Draw UI
        terminal.draw(|f| app.draw(f))?;
        
        // Handle events with timeout
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        
        if event::poll(timeout)? {
            let event = event::read()?;
            app.handle_event(event)?;
        }
        
        // Update on tick
        if last_tick.elapsed() >= tick_rate {
            app.update();
            last_tick = Instant::now();
        }
        
        // Exit condition
        if app.should_quit {
            break;
        }
    }
    
    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_app_creation() {
        let app = App::new("~/".to_string());
        assert!(!app.should_quit);
        assert_eq!(app.progress, 0.0);
    }
    
    #[test]
    fn test_app_update() {
        let mut app = App::new("~/".to_string());
        let initial_progress = app.progress;
        app.update();
        assert!(app.progress > initial_progress);
    }
    
    #[test]
    fn test_quit_on_q() {
        let mut app = App::new("~/".to_string());
        let key_event = KeyEvent::new(KeyCode::Char('q'), event::KeyModifiers::NONE);
        app.handle_key_event(key_event).unwrap();
        assert!(app.should_quit);
    }
}
