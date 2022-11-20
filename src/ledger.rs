use crate::model::{Account, Amount, Transaction};
use std::collections::HashMap;

#[derive(Default)]
pub struct Ledger {
    account_balances: HashMap<String, Amount>,
}

impl Ledger {
    pub fn process(&mut self, transactions: Vec<Transaction>) {
        for txn in transactions.into_iter() {
            for e in txn.entries() {
                let account = e.account();
                let currency = e.amount().currency();
                if !self.account_balances.contains_key(account.name()) {
                    self.account_balances
                        .insert(String::from(account.name()), Amount::new(*currency));
                }
                let amount = self.account_balances.get_mut(account.name()).unwrap();
                amount.set_number(amount.number() + e.amount().number());
            }
        }
    }

    pub fn get_balance(&self, account: &Account) -> Option<&Amount> {
        println!("{:?}", self.account_balances);
        self.account_balances.get(account.name())
    }
}
