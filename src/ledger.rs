use rust_decimal_macros::dec;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use crate::model::{Amount, Transaction};

#[derive(Default)]
pub struct Ledger {
    account_balances: HashMap<String, Amount>,
}

impl Ledger {
    pub fn add_transaction(&mut self, transaction: &Transaction) {
        for posting in transaction.postings() {
            let account = posting.account();
            let amount = self
                .account_balances
                .entry(account.clone())
                .or_insert_with(|| Amount::new(dec!(0), posting.amount().currency().to_string()));
            amount.plus(posting.amount());
        }
    }

    pub fn process(&mut self, transactions: &[Transaction]) {
        for txn in transactions.iter() {
            self.add_transaction(txn);
        }
    }

    pub fn get_balance(&self, account: &str) -> Option<&Amount> {
        self.account_balances.get(account)
    }

    pub fn dump_to_json(&self, filepath: &str) -> Result<(), Box<dyn Error>> {
        let file = File::create(Path::new(filepath))?;
        serde_json::to_writer(BufWriter::new(file), &self.account_balances)?;
        Ok(())
    }
}
