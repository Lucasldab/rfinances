use ratatui::Frame;
use ratatui::widgets::{Table, Row, Cell, Block, Borders};
use ratatui::layout::Constraint;

use crate::app::App;

pub fn render(frame: &mut Frame, app: &App) {
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
    .column_spacing(1);

    frame.render_widget(table, frame.area());
}
