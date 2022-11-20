use chrono::NaiveDate;
use rust_decimal_macros::dec;
use std::borrow::Borrow;
use tonic::{transport::Server, Request, Response, Status};

use crate::ledger::Ledger;
use crate::model::{Account, Amount, Currency, Posting, Transaction};
use crate::proto::acorn;
use crate::proto::acorn::acorn_server::{Acorn, AcornServer};
use crate::proto::acorn::{GetBalanceRequest, GetBalanceResponse};

mod error;
mod ledger;
mod model;
mod proto;

pub struct AcornImpl {
    acorn_ledger: Ledger,
}

impl AcornImpl {
    pub fn new(acorn_ledger: Ledger) -> Self {
        Self { acorn_ledger }
    }

    fn process(&mut self, transactions: Vec<Transaction>) {
        self.acorn_ledger.process(transactions);
    }

    fn get_balance(&self, account: &Account) -> Option<&Amount> {
        self.acorn_ledger.get_balance(account)
    }
}

#[tonic::async_trait]
impl Acorn for AcornImpl {
    async fn get_balance(
        &self,
        request: Request<GetBalanceRequest>,
    ) -> Result<Response<GetBalanceResponse>, Status> {
        let account = Account::from(request.into_inner().account);
        let amount_option = self.get_balance(&account);
        let reply = GetBalanceResponse {
            account,
            amount: match amount_option {
                None => None,
                Some(amount) => Some(acorn::Amount {
                    number: amount.number().to_string(),
                    currency: String::from(amount.currency()),
                }),
            },
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let mut acorn = AcornImpl::new(Ledger::default());

    acorn.process(Vec::from([Transaction::new(
        NaiveDate::from_ymd_opt(2022, 11, 1).unwrap(),
        "first txn",
        Vec::from([
            Posting::new(
                Account::from("Cash"),
                Amount::new(dec!(100), Currency::from("USD")),
            ),
            Posting::new(
                Account::from("Bank"),
                Amount::new(dec!(-100), Currency::from("USD")),
            ),
        ]),
    )]));

    println!("AcornServer listening on {}", addr);

    Server::builder()
        .add_service(AcornServer::new(acorn))
        .serve(addr)
        .await?;

    Ok(())
}
