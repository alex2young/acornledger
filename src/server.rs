use std::error::Error;
use tonic::{transport::Server, Request, Response, Status};

use crate::ledger::Ledger;
use crate::model::{Account, Amount, Transaction};
use crate::proto::acorn;
use crate::proto::acorn::acorn_server::{Acorn, AcornServer};
use crate::proto::acorn::{GetBalanceRequest, GetBalanceResponse};

mod error;
mod ledger;
mod model;
mod parser;
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
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let mut acorn = AcornImpl::new(Ledger::default());

    let tns = parser::parse_transactions_from_json("./data/input.json")?;
    acorn.process(tns);

    println!("AcornServer listening on {}", addr);

    Server::builder()
        .add_service(AcornServer::new(acorn))
        .serve(addr)
        .await?;

    Ok(())
}
