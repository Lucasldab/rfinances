mod app;
mod ui;
mod models;
mod storage;

use anyhow::Result;

fn main() -> Result<()> {
    ratatui::run(app::run)?;
    Ok(())
}
