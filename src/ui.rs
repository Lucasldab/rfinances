use ratatui::Frame;
use ratatui::widgets::{Table, Row, Cell, Block, Borders};
use ratatui::layout::{Layout, Direction, Constraint};
use ratatui::widgets::Paragraph;
use ratatui::style::Style;
use rust_decimal::Decimal;


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
                    Constraint::Length(1),
                ])
                .split(frame.area());
            frame.render_widget(
                Paragraph::new(app.input_description.clone())
                    .block(Block::default().title("Description").borders(Borders::ALL)),
                chunks[0]
            );
            frame.render_widget(
                Paragraph::new(app.input_amount.clone())
                    .block(Block::default().title("Amount").borders(Borders::ALL)),
                chunks[1]
            );
            frame.render_widget(
                Paragraph::new(app.input_category.clone())
                    .block(Block::default().title("Category").borders(Borders::ALL)),
                chunks[2]
            );
            frame.render_widget(
                Paragraph::new(" tab  next field | enter  save | esc  cancel")
                    .style(Style::default().reversed()),
                chunks[3]
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
