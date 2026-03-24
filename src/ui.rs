use ratatui::Frame;
use ratatui::widgets::{Table, Row, Cell, Block, Borders};
use ratatui::layout::Constraint;
use ratatui::style::Style;

use crate::app::App;

pub fn render(frame: &mut Frame, app: &mut App) {
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

    frame.render_stateful_widget(table, frame.area(), &mut app.table_state);
}
