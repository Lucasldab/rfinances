use std::time::Duration;

use anyhow::{Context, Result};
use ratatui::DefaultTerminal;
use crossterm::event::{self, KeyCode};

use crate::models::transaction::Transaction;
use crate::storage;

use crate::ui::render;

#[derive(Debug)]
pub struct App {
    pub transactions: Vec<Transaction>,
    pub selected: usize
}

pub fn run(terminal: &mut DefaultTerminal) -> Result<()> {
    let mut app = App {
        transactions: storage::load()?,
        selected: 0
    };

    loop {
        terminal.draw(|frame| render(frame, &app))?;
        if should_quit()? {
            break;
        }
    }
    Ok(())
}

fn should_quit() -> Result<bool> {
    if event::poll(Duration::from_millis(250)).context("event poll failed")? {
        let q_pressed = event::read()
            .context("event read failed")?
            .as_key_press_event()
            .is_some_and(|key| key.code == KeyCode::Char('q'));
        return Ok(q_pressed);
    }
    Ok(false)
}
