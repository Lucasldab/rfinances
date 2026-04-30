# rfinances — Codebase Scan

## 1. The `~` Directory

### What it is

`~` is a **directory** at the repo root containing:

```
~/.local/share/rfinances/data.json
```

That JSON file holds two test transactions (one Income, one Expense) that were saved during early development.

### Root cause

An earlier version of the storage code constructed the data path by concatenating a literal `"~"` string with `/.local/share/rfinances/data.json`. Because Rust does not shell-expand `~`, `std::fs::create_dir_all` created a literal directory named `~` inside the working directory (i.e. the repo root). The directory — and the data file inside it — were then accidentally committed to git.

Evidence: the commit `d2f0575 fix: use dirs crate for proper data path` replaced that literal approach with `dirs::data_local_dir()`, which resolves to the real user home (`~/.local/share` on Linux). The `~` directory artifact was never cleaned up.

The directory is **tracked by git** (`git ls-files` returns `~/.local/share/rfinances/data.json`).

### What to do

1. **Remove from git tracking and delete locally:**
   ```bash
   git rm -r "~"
   git commit -m "chore: remove accidentally committed ~ directory"
   ```
2. **Add to `.gitignore`** as a guard (the literal name `~` is unusual enough to warrant it):
   ```
   ~/
   ```
   Add this line to `.gitignore` so a recurrence does not slip in unnoticed.

Do **not** rename — the directory has no valid purpose; deleting is correct.

---

## 2. Modules & Responsibilities

| File | Responsibility |
|---|---|
| `src/main.rs` | Binary entry point. Delegates to `ratatui::run(app::run)`. |
| `src/app.rs` | `App` state struct (`transactions`, `screen`, input buffers, cursor). `Screen` enum (`List`, `AddTransaction`, `Dashboard`). `run()` drives the event loop; `handle_input()` dispatches key events per screen. |
| `src/ui.rs` | All rendering. `render()` matches on `Screen` and draws the appropriate layout using ratatui widgets (Table, Paragraph, Block). Dashboard totals (income, expense, balance) are computed inline here. |
| `src/storage.rs` | Persistence. `load()` reads `data.json` via `dirs::data_local_dir()`; `save()` serialises the transaction slice with `serde_json`. Creates the directory if missing. Has one integration test (`test_save_and_load`). |
| `src/models/transaction.rs` | Data types. `Transaction` (amount, description, category, date, type) and `TransactionType` (`Income` / `Expense`), both serde-serialisable. |
| `src/models/mod.rs` | Re-exports the `transaction` module. |

---

## 3. One Missing Test

**Target:** dashboard balance calculation in `src/ui.rs`.

The income, expense, and balance totals are computed inline inside `render()` with no abstraction:

```rust
let total_income: Decimal = app.transactions.iter()
    .filter(|t| t.transaction_type == TransactionType::Income)
    .map(|t| t.amount).sum();
let total_expense: Decimal = ...;
let total_balance = total_income - total_expense;
```

This logic is untested. Extracting it into a free function and adding a unit test would catch regressions (e.g. if filter predicate is accidentally swapped):

```rust
// src/ui.rs
pub fn compute_totals(transactions: &[Transaction]) -> (Decimal, Decimal, Decimal) {
    let income: Decimal = transactions.iter()
        .filter(|t| t.transaction_type == TransactionType::Income)
        .map(|t| t.amount).sum();
    let expense: Decimal = transactions.iter()
        .filter(|t| t.transaction_type == TransactionType::Expense)
        .map(|t| t.amount).sum();
    (income, expense, income - expense)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;
    use chrono::NaiveDate;
    use crate::models::transaction::{Transaction, TransactionType};

    #[test]
    fn test_compute_totals() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let transactions = vec![
            Transaction { amount: Decimal::new(100, 0), description: "salary".into(),
                category: "income".into(), date, transaction_type: TransactionType::Income },
            Transaction { amount: Decimal::new(40, 0), description: "rent".into(),
                category: "housing".into(), date, transaction_type: TransactionType::Expense },
        ];
        let (income, expense, balance) = compute_totals(&transactions);
        assert_eq!(income, Decimal::new(100, 0));
        assert_eq!(expense, Decimal::new(40, 0));
        assert_eq!(balance, Decimal::new(60, 0));
    }
}
```
