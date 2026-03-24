use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub amount: Decimal,
    pub description: String,
    pub category: String,
    pub date: NaiveDate,
    pub transaction_type: TransactionType
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TransactionType {
    Income,
    Expense,
}
