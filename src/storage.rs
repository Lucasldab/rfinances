use anyhow::Result;
use crate::models::transaction::Transaction;

pub fn load() -> Result<Vec<Transaction>> {
    let path = dirs::data_local_dir()
        .expect("could not find data directory")
        .join("rfinances")
        .join("data.json");

    if !path.exists() {
        Ok(vec![])
    }else {
        let contents = std::fs::read_to_string(path)?;
        let transactions = serde_json::from_str(&contents)?;
        Ok(transactions)
    }

}

pub fn save(transactions: &[Transaction]) -> Result<()> {
    let path = dirs::data_local_dir()
        .expect("could not find data directory")
        .join("rfinances")
        .join("data.json");
    std::fs::create_dir_all(path.parent().unwrap())?;
    serde_json::to_writer_pretty(std::fs::File::create(path)?, transactions)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::transaction::Transaction;
    use rust_decimal::Decimal;
    use chrono::NaiveDate;
    use crate::models::transaction::TransactionType;

    #[test]
    fn test_save_and_load() {
    let transactions = vec![Transaction {
    amount: Decimal::new(10, 0),
    description: "test".to_string(),
    category: "test".to_string(),
    date:NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
    transaction_type: TransactionType::Income,
    }];

    save(&transactions).unwrap();
    let loaded = load().unwrap();
    assert_eq!(loaded.len(), 1);
    }
}
