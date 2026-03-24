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
    pub table_state: TableState,
    pub screen: Screen,
    pub input_amount: String,
    pub input_description: String,
    pub input_category: String,
    pub input_field: usize
}

#[derive(Debug)]
pub enum Screen {
    List,
    AddTransaction,
}

pub fn run(terminal: &mut DefaultTerminal) -> Result<()> {
    let mut app = App {
        transactions: storage::load()?,
        table_state: TableState::default(),
        screen: Screen::List,
        input_amount: String::new(),
        input_description: String::new(),
        input_category: String::new(),
        input_field: 0
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
                KeyCode::Char('a') => app.screen = Screen::AddTransaction,
                _ => {}
            }
        }
    }
    Ok(false)
}
