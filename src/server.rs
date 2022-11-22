use chrono::NaiveDate;
use rust_decimal::Decimal;
use std::error::Error;
use std::sync::Mutex;
use tonic::{transport::Server, Request, Response, Status};

use crate::ledger::Ledger;
use crate::model::{Amount, Posting, Transaction};
use crate::proto::acorn;
use crate::proto::acorn::acorn_server::{Acorn, AcornServer};
use crate::proto::acorn::{AddTransactionRequest, Empty, GetBalanceRequest, GetBalanceResponse};

mod error;
mod ledger;
mod model;
mod parser;
mod proto;

pub struct AcornImpl {
    ledger: Mutex<Ledger>,
}

impl AcornImpl {
    pub fn new(ledger: Ledger) -> Self {
        Self {
            ledger: Mutex::new(ledger),
        }
    }
}

#[tonic::async_trait]
impl Acorn for AcornImpl {
    async fn add_transaction(
        &self,
        request: Request<AddTransactionRequest>,
    ) -> Result<Response<Empty>, Status> {
        let input_txn = request.into_inner().transaction.unwrap_or_default();
        let date = input_txn.date.unwrap_or_default();
        let transaction = Transaction::new(
            NaiveDate::from_ymd_opt(date.year as i32, date.month, date.day).unwrap_or_default(),
            input_txn.description.as_str(),
            input_txn
                .postings
                .into_iter()
                .map(|p| {
                    Posting::new(
                        p.account,
                        Amount::new(
                            Decimal::from_str_exact(p.amount.as_ref().unwrap().number.as_str())
                                .unwrap_or_default(),
                            p.amount.unwrap_or_default().currency,
                        ),
                    )
                })
                .collect(),
        )
        .unwrap();
        self.ledger.lock().unwrap().add_transaction(&transaction);
        Ok(Response::new(Empty {}))
    }

    async fn dump_transactions(&self, _request: Request<Empty>) -> Result<Response<Empty>, Status> {
        match self
            .ledger
            .lock()
            .unwrap()
            .dump_to_json("./data/output.json")
        {
            Ok(_) => Ok(Response::new(Empty {})),
            Err(e) => Err(Status::data_loss(e.to_string())),
        }
    }

    async fn get_balance(
        &self,
        request: Request<GetBalanceRequest>,
    ) -> Result<Response<GetBalanceResponse>, Status> {
        let account = request.into_inner().account;
        let reply = GetBalanceResponse {
            account: account.clone(),
            amount: self
                .ledger
                .lock()
                .unwrap()
                .get_balance(&account)
                .map(|amount| acorn::Amount {
                    number: amount.number().to_string(),
                    currency: String::from(amount.currency()),
                }),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let acorn = AcornImpl::new(Ledger::default());

    let tns = parser::parse_transactions_from_json("./data/input.json")?;
    acorn.ledger.lock().unwrap().process(&tns);

    println!("AcornServer listening on {}", addr);

    Server::builder()
        .add_service(AcornServer::new(acorn))
        .serve(addr)
        .await?;

    Ok(())
}
