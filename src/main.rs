mod app;
mod ui;

use anyhow::Result;

fn main() -> Result<()> {
    ratatui::run(app::run)?;
    Ok(())
}
