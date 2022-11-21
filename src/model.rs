use chrono::NaiveDate;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::collections::HashMap;
use std::error::Error;

use crate::error::AcornError;

pub type Account = String;
pub type Currency = String;
pub type Meta = Option<HashMap<String, String>>;

#[derive(Debug)]
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

pub struct Posting {
    account: Account,
    amount: Amount,
    meta: Meta,
}

impl Posting {
    pub fn account(&self) -> &str {
        &self.account
    }
    pub fn amount(&self) -> &Amount {
        &self.amount
    }

    pub fn new(account: Account, amount: Amount) -> Self {
        Self {
            account,
            amount,
            meta: None,
        }
    }
}

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
