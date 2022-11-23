use chrono::NaiveDate;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

use crate::error::AcornError;
use crate::proto;

#[derive(Debug, Serialize, Deserialize)]
pub struct Amount {
    number: Decimal,
    currency: String,
}

impl Amount {
    pub fn number(&self) -> Decimal {
        self.number
    }
    pub fn currency(&self) -> &str {
        &self.currency
    }

    pub fn new(number: Decimal, currency: String) -> Self {
        Self { number, currency }
    }

    pub fn plus(&mut self, other: &Amount) {
        self.number += other.number();
    }

    fn from(amount: &proto::acorn::Amount) -> Self {
        Self {
            number: Decimal::from_str_exact(&amount.number).unwrap(),
            currency: String::from(&amount.currency),
        }
    }

    pub fn to_message(&self) -> proto::acorn::Amount {
        proto::acorn::Amount {
            number: self.number.to_string(),
            currency: String::from(&self.currency),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Posting {
    account: String,
    amount: Amount,
}

impl Posting {
    pub fn account(&self) -> &String {
        &self.account
    }
    pub fn amount(&self) -> &Amount {
        &self.amount
    }

    pub fn new(account: String, amount: Amount) -> Self {
        Self { account, amount }
    }

    fn from(posting: &proto::acorn::Posting) -> Self {
        Self {
            account: String::from(&posting.account),
            amount: Amount::from(posting.amount.as_ref().unwrap()),
        }
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

    pub fn date(&self) -> NaiveDate {
        self.date
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

    pub fn from(transaction: proto::acorn::Transaction) -> Result<Self, Box<dyn Error>> {
        Self::new(
            transaction
                .date
                .map(|date| {
                    NaiveDate::from_ymd_opt(date.year as i32, date.month, date.day).unwrap()
                })
                .unwrap(),
            &transaction.description,
            transaction.postings.iter().map(Posting::from).collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_postings() {
        assert!(Transaction::validate_postings(&Vec::from([
            Posting::new(
                String::from("Cash"),
                Amount::new(dec!(100.00), String::from("C")),
            ),
            Posting::new(
                String::from("Bank"),
                Amount::new(dec!(-100.00), String::from("C")),
            ),
        ])));

        assert!(!Transaction::validate_postings(&Vec::from([
            Posting::new(
                String::from("Cash"),
                Amount::new(dec!(100.00), String::from("C")),
            ),
            Posting::new(
                String::from("Bank"),
                Amount::new(dec!(-101.00), String::from("C")),
            ),
        ])));

        assert!(Transaction::validate_postings(&Vec::from([
            Posting::new(
                String::from("Cash"),
                Amount::new(dec!(100.00), String::from("C")),
            ),
            Posting::new(
                String::from("Bank"),
                Amount::new(dec!(-100.00), String::from("C")),
            ),
            Posting::new(
                String::from("Expenses"),
                Amount::new(dec!(220.00), String::from("X")),
            ),
            Posting::new(
                String::from("Income"),
                Amount::new(dec!(-220.00), String::from("X")),
            ),
        ])));
    }
}
