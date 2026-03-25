use std::time::Duration;
use rust_decimal::Decimal;

use anyhow::{Context, Result};
use ratatui::DefaultTerminal;
use ratatui::widgets::TableState;
use crossterm::event::{self, KeyCode};

use crate::models::transaction::{Transaction, TransactionType};
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
    pub input_field: usize,
    pub input_type: TransactionType
}

#[derive(Debug)]
pub enum Screen {
    List,
    AddTransaction,
    Dashboard,
}

pub fn run(terminal: &mut DefaultTerminal) -> Result<()> {
    let mut app = App {
        transactions: storage::load()?,
        table_state: TableState::default(),
        screen: Screen::List,
        input_amount: String::new(),
        input_description: String::new(),
        input_category: String::new(),
        input_field: 0,
        input_type: TransactionType::Expense
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
            match app.screen {
                Screen::List => {
                    match key.code {
                        KeyCode::Char('q') => return Ok(true),
                        KeyCode::Down | KeyCode::Char('j') => app.table_state.select_next(),
                        KeyCode::Up | KeyCode::Char('k') => app.table_state.select_previous(),
                        KeyCode::Char('a') => app.screen = Screen::AddTransaction,
                        KeyCode::Char('d') => app.screen = Screen::Dashboard,
                        _ => {}
                    }
                }
                Screen::AddTransaction => {
                    match key.code {
                        KeyCode::Esc => {
                            app.screen = Screen::List;
                            app.input_description.clear();
                            app.input_amount.clear();
                            app.input_category.clear();
                            app.input_field = 0;
                        }
                        KeyCode::Tab => {
                            app.input_field = (app.input_field + 1) % 4;
                        }
                        KeyCode::Enter => {
                            let amount = match app.input_amount.parse::<Decimal>() {
                                Ok(a) => a,
                                Err(_) => return Ok(false),
                            };
                            let transaction = Transaction {
                                amount,
                                description: app.input_description.clone(),
                                category: app.input_category.clone(),
                                date: chrono::Local::now().date_naive(),
                                transaction_type: app.input_type.clone(),
                            };
                            app.transactions.push(transaction);
                            storage::save(&app.transactions)?;
                            app.input_description.clear();
                            app.input_amount.clear();
                            app.input_category.clear();
                            app.input_field = 0;
                            app.screen = Screen::List;
                            app.input_type = TransactionType::Expense;
                        }
                        KeyCode::Backspace => {
                            match app.input_field {
                                0 => { app.input_description.pop(); }
                                1 => { app.input_amount.pop(); }
                                2 => { app.input_category.pop(); }
                                _ => {}
                            }
                    }
                        KeyCode::Char(c) => {
                            match app.input_field {
                                0 => app.input_description.push(c),
                                1 => app.input_amount.push(c),
                                2 => app.input_category.push(c),
                                3 => {
                                    if c == 't' {
                                        app.input_type = match app.input_type {
                                            TransactionType::Income => TransactionType::Expense,
                                            TransactionType::Expense => TransactionType::Income,
                                        };
                                    }
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                Screen::Dashboard => {
                    match key.code {
                        KeyCode::Char('q') => return Ok(true),
                        KeyCode::Esc => app.screen = Screen::List,
                        _ => {}
                    }
                }
            }
        }
    }
    Ok(false)
}
