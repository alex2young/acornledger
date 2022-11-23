use chrono::NaiveDate;
use serde::{Serialize, Serializer};
use std::collections::BTreeMap;
use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::model::{Amount, Transaction};

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct TxKey {
    date: NaiveDate,
    id: usize,
}

impl TxKey {
    fn new(date: NaiveDate, id: usize) -> Self {
        Self { date, id }
    }

    fn from(date: NaiveDate) -> Self {
        Self { date, id: 0 }
    }
}

impl Serialize for TxKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{}:{}", self.date, self.id))
    }
}

#[derive(Default)]
pub struct Ledger {
    counter: AtomicUsize,
    transactions: BTreeMap<TxKey, Transaction>,
}

impl Ledger {
    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.insert(
            TxKey::new(
                transaction.date(),
                self.counter.fetch_add(1, Ordering::Relaxed),
            ),
            transaction,
        );
    }

    pub fn add_transactions(&mut self, transactions: Vec<Transaction>) {
        for txn in transactions.into_iter() {
            self.add_transaction(txn);
        }
    }

    pub fn get_latest_balance(&self, account: &str) -> Option<Amount> {
        self.get_balance(account, NaiveDate::MIN, NaiveDate::MAX)
    }

    pub fn get_balance(&self, account: &str, begin: NaiveDate, end: NaiveDate) -> Option<Amount> {
        let number = self
            .transactions
            .range(TxKey::from(begin)..TxKey::from(end))
            .flat_map(|(_, t)| t.postings().iter())
            .filter(|&p| p.account() == account)
            .map(|p| p.amount().number())
            .reduce(|acc, p| (acc + p))?;
        Some(Amount::new(number, "USD".to_string()))
    }

    pub fn dump_to_json(&self, filepath: &str) -> Result<(), Box<dyn Error>> {
        let file = File::create(Path::new(filepath))?;
        serde_json::to_writer(BufWriter::new(file), &self.transactions)?;
        Ok(())
    }
}
