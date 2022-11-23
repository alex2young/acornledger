use std::error::Error;
use std::sync::Mutex;
use tonic::{transport::Server, Request, Response, Status};

use crate::ledger::Ledger;
use crate::model::Transaction;

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
        let transaction_result =
            Transaction::from(request.into_inner().transaction.unwrap_or_default());
        match transaction_result {
            Ok(transaction) => {
                self.ledger.lock().unwrap().add_transaction(transaction);
                Ok(Response::new(Empty {}))
            }
            Err(_) => Err(Status::data_loss("".to_string())),
        }
    }

    async fn dump_to_json(&self, _request: Request<Empty>) -> Result<Response<Empty>, Status> {
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
                .get_latest_balance(&account)
                .map(|amount| amount.to_message()),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let acorn = AcornImpl::new(Ledger::default());

    let tns = parser::parse_transactions_from_json("./data/input.json")?;
    acorn.ledger.lock().unwrap().add_transactions(tns);

    println!("AcornServer listening on {}", addr);

    Server::builder()
        .add_service(AcornServer::new(acorn))
        .serve(addr)
        .await?;

    Ok(())
}
