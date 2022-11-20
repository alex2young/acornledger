use tonic::Request;

use crate::proto::acorn::acorn_client::AcornClient;
use crate::proto::acorn::GetBalanceRequest;

mod proto;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = AcornClient::connect("http://[::1]:50051").await?;

    let request = Request::new(GetBalanceRequest {
        account: String::from("Cash"),
    });

    let response = client.get_balance(request).await?;
    println!("{:?}", response);

    Ok(())
}
