use proto::seer_client::SeerClient;

mod proto {
  tonic::include_proto!("seer");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://[::1]:50051";
    let mut client = SeerClient::connect(url).await?;

    let req = proto::ConversionRequest { from: "a".to_string(), to: "b".to_string(), value: 13.08 };
    let request = tonic::Request::new(req);

    let response = client.convert(request).await?;

    println!("Response: {:?}", response.get_ref().result);

    Ok(())
}
