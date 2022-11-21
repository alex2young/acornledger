use chrono::NaiveDate;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

use crate::error::AcornError;

pub type Account = String;
pub type Currency = String;
// pub type Meta = Option<HashMap<String, String>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Amount {
    number: Decimal,
    currency: Currency,
}

impl Amount {
    pub fn number(&self) -> Decimal {
        self.number
    }
    pub fn currency(&self) -> &str {
        &self.currency
    }

    pub fn new(number: Decimal, currency: Currency) -> Self {
        Self { number, currency }
    }

    pub fn plus(&mut self, other: &Amount) {
        self.number += other.number();
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Posting {
    account: Account,
    amount: Amount,
}

impl Posting {
    pub fn account(&self) -> &Account {
        &self.account
    }
    pub fn amount(&self) -> &Amount {
        &self.amount
    }

    pub fn new(account: Account, amount: Amount) -> Self {
        Self { account, amount }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    date: NaiveDate,
    description: String,
    postings: Vec<Posting>,
}

impl Transaction {
    pub fn postings(&self) -> &Vec<Posting> {
        &self.postings
    }

    pub fn new(
        date: NaiveDate,
        description: &str,
        postings: Vec<Posting>,
    ) -> Result<Self, Box<dyn Error>> {
        if !Transaction::validate_postings(&postings) {
            Err(AcornError)?;
        }
        Ok(Self {
            date,
            description: String::from(description),
            postings,
        })
    }

    fn validate_postings(postings: &Vec<Posting>) -> bool {
        let mut m = HashMap::new();
        for posting in postings {
            let mut number = m.entry(posting.amount().currency()).or_insert(dec!(0));
            number += posting.amount().number();
        }
        for (_, number) in m.iter() {
            if !number.is_zero() {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_postings() {
        assert!(Transaction::validate_postings(&Vec::from([
            Posting::new(
                Account::from("Cash"),
                Amount::new(dec!(100.00), Currency::from("C")),
            ),
            Posting::new(
                Account::from("Bank"),
                Amount::new(dec!(-100.00), Currency::from("C")),
            ),
        ])));

        assert!(!Transaction::validate_postings(&Vec::from([
            Posting::new(
                Account::from("Cash"),
                Amount::new(dec!(100.00), Currency::from("C")),
            ),
            Posting::new(
                Account::from("Bank"),
                Amount::new(dec!(-101.00), Currency::from("C")),
            ),
        ])));

        assert!(Transaction::validate_postings(&Vec::from([
            Posting::new(
                Account::from("Cash"),
                Amount::new(dec!(100.00), Currency::from("C")),
            ),
            Posting::new(
                Account::from("Bank"),
                Amount::new(dec!(-100.00), Currency::from("C")),
            ),
            Posting::new(
                Account::from("Expenses"),
                Amount::new(dec!(220.00), Currency::from("X")),
            ),
            Posting::new(
                Account::from("Income"),
                Amount::new(dec!(-220.00), Currency::from("X")),
            ),
        ])));
    }
}
