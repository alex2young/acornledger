use chrono::NaiveDate;
use rust_decimal::Decimal;

#[derive(Eq, PartialEq, Hash)]
pub struct Account {
    name: String,
}

impl Account {
    pub fn new(name: String) -> Self {
        Self { name }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Currency {
    CNY,
    USD,
    JPY,
}

impl Currency {
    pub fn to_string(&self) -> String {
        String::from(match self {
            Currency::CNY => "CNY",
            Currency::USD => "USD",
            Currency::JPY => "JPY",
        })
    }
}

#[derive(Debug)]
pub struct Amount {
    number: Decimal,
    currency: Currency,
}

impl Amount {
    pub fn number(&self) -> Decimal {
        self.number
    }
    pub fn currency(&self) -> &Currency {
        &self.currency
    }
    pub fn new(currency: Currency) -> Self {
        Self {
            number: Decimal::new(0, 0),
            currency,
        }
    }
    pub fn from(number: Decimal, currency: Currency) -> Self {
        Self { number, currency }
    }
    pub fn set_number(&mut self, number: Decimal) {
        self.number = number;
    }
}

pub struct TxEntry {
    account: Account,
    amount: Amount,
}

impl TxEntry {
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

pub struct Transaction {
    date: NaiveDate,
    entries: Vec<TxEntry>,
}

impl Transaction {
    pub fn date(&self) -> NaiveDate {
        self.date
    }
    pub fn entries(&self) -> &Vec<TxEntry> {
        &self.entries
    }
    pub fn new(date: NaiveDate, entries: Vec<TxEntry>) -> Self {
        Self { date, entries }
    }
}
