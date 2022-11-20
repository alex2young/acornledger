use tonic::Request;

use proto::acorn::acorn_client::AcornClient;
use proto::acorn::GetBalanceRequest;

pub mod proto;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = AcornClient::connect("http://[::1]:50051").await?;

    let request = Request::new(GetBalanceRequest {
        account: String::from("Cash"),
    });

    let response = client.get_balance(request).await?;

    println!("ACCOUNT = {:?}", response.get_ref().account);
    println!(
        "NUMBER = {:?}",
        response.get_ref().amount.as_ref().unwrap().number
    );
    println!(
        "CURRENCY = {:?}",
        response.get_ref().amount.as_ref().unwrap().currency
    );

    Ok(())
}
