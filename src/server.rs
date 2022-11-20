use chrono::NaiveDate;
use rust_decimal_macros::dec;
use std::borrow::Borrow;
use tonic::{transport::Server, Request, Response, Status};

use crate::ledger::Ledger;
use crate::model::{Currency, Transaction, TxEntry};
use model::Account;
use proto::acorn::acorn_server::{Acorn, AcornServer};
use proto::acorn::{Amount, GetBalanceRequest, GetBalanceResponse};

mod error;
mod ledger;
mod model;
mod proto;

pub struct AcornImpl {
    acorn_ledger: Ledger,
}

#[tonic::async_trait]
impl Acorn for AcornImpl {
    async fn get_balance(
        &self,
        request: Request<GetBalanceRequest>,
    ) -> Result<Response<GetBalanceResponse>, Status> {
        let account = Account::new(request.get_ref().to_owned().account);
        let amount = self.acorn_ledger.get_balance(account.borrow()).unwrap();

        let reply = GetBalanceResponse {
            account: String::from(account.name()),
            amount: Some(Amount {
                number: amount.number().to_string(),
                currency: amount.currency().to_string(),
            }),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let acorn_ledger = Ledger::default();
    let mut acorn = AcornImpl { acorn_ledger };

    acorn.acorn_ledger.process(Vec::from([Transaction::new(
        NaiveDate::from_ymd_opt(2022, 11, 1).unwrap(),
        Vec::from([
            TxEntry::new(
                Account::new(String::from("Cash")),
                model::Amount::from(dec!(100), Currency::USD),
            ),
            TxEntry::new(
                Account::new(String::from("Bank")),
                model::Amount::from(dec!(-100), Currency::USD),
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
