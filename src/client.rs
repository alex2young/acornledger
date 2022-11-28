use std::error::Error;
use tonic::Request;

use crate::proto::acorn::acorn_client::AcornClient;
use crate::proto::acorn::{
    AddTransactionRequest, Amount, Date, Empty, GetBalanceRequest, GetLatestBalanceRequest,
    Posting, Transaction,
};

mod proto;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut client = AcornClient::connect("http://[::1]:50051").await?;

    let resp1 = client
        .get_latest_balance(Request::new(GetLatestBalanceRequest {
            account: String::from("Cash"),
        }))
        .await?;
    println!("{:?}", resp1);

    let resp2 = client
        .add_transaction(Request::new(AddTransactionRequest {
            transaction: Some(Transaction {
                date: Some(Date {
                    year: 2022,
                    month: 11,
                    day: 21,
                }),
                description: "我从远方赶来".to_string(),
                postings: Vec::from([
                    Posting {
                        account: "Cash".to_string(),
                        amount: Some(Amount {
                            number: "16.00".to_string(),
                            currency: "USD".to_string(),
                        }),
                    },
                    Posting {
                        account: "Income:RedPocket".to_string(),
                        amount: Some(Amount {
                            number: "-16.00".to_string(),
                            currency: "USD".to_string(),
                        }),
                    },
                ]),
            }),
        }))
        .await?;
    println!("{:?}", resp2);

    let resp3 = client
        .get_latest_balance(Request::new(GetLatestBalanceRequest {
            account: String::from("Cash"),
        }))
        .await?;
    println!("{:?}", resp3);

    let resp4 = client
        .get_balance(Request::new(GetBalanceRequest {
            account: "Cash".to_string(),
            begin: Some(Date {
                year: 2022,
                month: 11,
                day: 01,
            }),
            end: Some(Date {
                year: 2022,
                month: 11,
                day: 16,
            }),
        }))
        .await?;
    println!("{:?}", resp4);

    let resp5 = client.dump_transactions(Request::new(Empty {})).await?;
    println!("{:?}", resp5);

    Ok(())
}
