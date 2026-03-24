use anyhow::Result;
use crate::models::transaction::Transaction;

pub fn load() -> Result<Vec<Transaction>> {
    let path = std::path::Path::new("~/.local/share/rfinances/data.json");

    if !path.exists() {
        Ok(vec![])
    }else {
        let contents = std::fs::read_to_string(path)?;
        let transactions = serde_json::from_str(&contents)?;
        Ok(transactions)
    }

}

pub fn save(transactions: &[Transaction]) -> Result<()> {
    let path = std::path::Path::new("~/.local/share/rfinances/data.json");
    std::fs::create_dir_all(path.parent().unwrap())?;
    serde_json::to_writer_pretty(std::fs::File::create(path)?, transactions)?;
    Ok(())
}
