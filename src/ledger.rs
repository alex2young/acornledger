use crate::model::{Account, Amount, Currency, Transaction};
use rust_decimal_macros::dec;
use std::collections::HashMap;

#[derive(Default)]
pub struct Ledger {
    account_balances: HashMap<Account, Amount>,
}

impl Ledger {
    pub fn process(&mut self, transactions: Vec<Transaction>) {
        for txn in transactions.into_iter() {
            for posting in txn.postings() {
                let account = posting.account();
                let currency = posting.amount().currency();
                if !self.account_balances.contains_key(account) {
                    self.account_balances.insert(
                        Account::from(account),
                        Amount::new(dec!(0), Currency::from(currency)),
                    );
                }
                let amount = self.account_balances.get_mut(account).unwrap();
                amount.plus(posting.amount());
            }
        }
    }

    pub fn get_balance(&self, account: &Account) -> Option<&Amount> {
        self.account_balances.get(account)
    }
}
