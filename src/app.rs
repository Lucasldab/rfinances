use std::time::Duration;

use anyhow::{Context, Result};
use ratatui::DefaultTerminal;
use ratatui::widgets::TableState;
use crossterm::event::{self, KeyCode};

use crate::models::transaction::Transaction;
use crate::storage;

use crate::ui::render;

#[derive(Debug)]
pub struct App {
    pub transactions: Vec<Transaction>,
    pub table_state: TableState
}

pub fn run(terminal: &mut DefaultTerminal) -> Result<()> {
    let mut app = App {
        transactions: storage::load()?,
        table_state: TableState::default(),
    };

    loop {
        terminal.draw(|frame| render(frame, &mut app))?;
        if handle_input(&mut app)? {
            break;
        }
    }
    Ok(())
}

fn handle_input(app: &mut App) -> Result<bool> {
    if event::poll(Duration::from_millis(250)).context("event poll failed")? {
        if let event::Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(true),
                KeyCode::Down | KeyCode::Char('j') => app.table_state.select_next(),
                KeyCode::Up | KeyCode::Char('k') => app.table_state.select_previous(),
                _ => {}
            }
        }
    }
    Ok(false)
}
