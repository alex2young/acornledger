use rust_decimal_macros::dec;
use std::collections::HashMap;

use crate::model::{Account, Amount, Transaction};

#[derive(Default)]
pub struct Ledger {
    account_balances: HashMap<Account, Amount>,
}

impl Ledger {
    pub fn process(&mut self, transactions: Vec<Transaction>) {
        for txn in transactions.into_iter() {
            for posting in txn.postings() {
                let account = posting.account();
                let amount = self
                    .account_balances
                    .entry(account.clone())
                    .or_insert_with(|| {
                        Amount::new(dec!(0), posting.amount().currency().to_string())
                    });
                amount.plus(posting.amount());
            }
        }
    }

    pub fn get_balance(&self, account: &Account) -> Option<&Amount> {
        self.account_balances.get(account)
    }
}
