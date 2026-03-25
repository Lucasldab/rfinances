use ratatui::Frame;
use ratatui::widgets::{Table, Row, Cell, Block, Borders};
use ratatui::layout::{Layout, Direction, Constraint};
use ratatui::widgets::Paragraph;
use ratatui::style::Style;
use rust_decimal::Decimal;
use ratatui::style::Color;

use crate::app::Screen;
use crate::app::App;
use crate::models::transaction::TransactionType;

pub fn render(frame: &mut Frame, app: &mut App) {
    match app.screen {
        Screen::List => {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(0),
                    Constraint::Length(1),
                ])
                .split(frame.area());

            let rows = app.transactions.iter().map(|t| {
                Row::new(vec![
                    Cell::from(t.date.to_string()),
                    Cell::from(t.description.clone()),
                    Cell::from(t.category.clone()),
                    Cell::from(t.amount.to_string()),
                    Cell::from(format!("{:?}", t.transaction_type)),
                ])
            });

            let table = Table::new(rows, [
                Constraint::Length(10),
                Constraint::Length(20),
                Constraint::Length(15),
                Constraint::Length(10),
                Constraint::Length(10),
            ])
                .header(Row::new(vec!["Date", "Description", "Category", "Amount", "Type"]))
                .block(Block::default().title("Transactions").borders(Borders::ALL))
                .column_spacing(1)
                .row_highlight_style(Style::default().reversed());
            frame.render_stateful_widget(table, layout[0], &mut app.table_state);

            frame.render_widget(
                Paragraph::new(" a  add | d  dashboard | j/k  navigate | q  quit")
                    .style(Style::default().reversed()),
                layout[1]
            );
        }
        Screen::AddTransaction => {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(1),
                ])
                .split(frame.area());
            let description_block = if app.input_field == 0 {
                Block::default().title("Description").borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Yellow))
            } else {
                Block::default().title("Description").borders(Borders::ALL)
            };
            frame.render_widget(
                Paragraph::new(app.input_description.clone())
                    .block(description_block),
                chunks[0]
            );
             let amount_block = if app.input_field == 1 {
                Block::default().title("Amount").borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Yellow))
            } else {
                Block::default().title("Amount").borders(Borders::ALL)
            };
            frame.render_widget(
                Paragraph::new(app.input_amount.clone())
                    .block(amount_block),
                chunks[1]
            );
             let category_block = if app.input_field == 2 {
                Block::default().title("Category").borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Yellow))
            } else {
                Block::default().title("Category").borders(Borders::ALL)
            };
            frame.render_widget(
                Paragraph::new(app.input_category.clone())
                    .block(category_block),
                chunks[2]
            );
             let type_block = if app.input_field == 3 {
                Block::default().title("Type").borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Yellow))
            } else {
                Block::default().title("Type").borders(Borders::ALL)
            };
            frame.render_widget(
                Paragraph::new(format!("Type: {:?} (press t to toggle)", app.input_type))
                    .block(type_block),
                chunks[3]
            );
            frame.render_widget(
                Paragraph::new(" tab  next field | enter  save | esc  cancel | t  toggle type")
                    .style(Style::default().reversed()),
                chunks[4]
            );
        }
        Screen::Dashboard => {
            let total_income: Decimal = app.transactions.iter().filter(|t| t.transaction_type == TransactionType::Income).map(|t| t.amount).sum();
            let total_expense: Decimal = app.transactions.iter().filter(|t| t.transaction_type == TransactionType::Expense).map(|t| t.amount).sum();
            let total_balance   = total_income - total_expense;

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(1),
                ])
                .split(frame.area());
            frame.render_widget(
                Paragraph::new(format!("Income: {}", total_income))
                    .block(Block::default().title("Dashboard").borders(Borders::ALL)),
                chunks[0]
            );

            frame.render_widget(
                Paragraph::new(format!("Expenses: {}", total_expense))
                    .block(Block::default().title("Dashboard").borders(Borders::ALL)),
                chunks[1]
            );

            frame.render_widget(
                Paragraph::new(format!("Balance: {}", total_balance))
                    .block(Block::default().title("Dashboard").borders(Borders::ALL)),
                chunks[2]
            );

            frame.render_widget(
                Paragraph::new(" esc  back | q  quit")
                    .style(Style::default().reversed()),
                chunks[3]
            );
        }
    }
}
