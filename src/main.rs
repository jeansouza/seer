use proto::seer_server::{Seer, SeerServer};
use tonic::transport::Server;
use std::collections::HashMap;

mod proto {
  tonic::include_proto!("seer");

  pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
    tonic::include_file_descriptor_set!("seer_descriptor");
}

#[derive(Debug, Default)]
struct SeerService {}

#[tonic::async_trait]
impl Seer for SeerService {
  async fn convert(
    &self,
    request: tonic::Request<proto::ConversionRequest>,
  ) -> Result<tonic::Response<proto::ConversionResponse>, tonic::Status> {
    println!("Got a request: {:?}", request);

    let input = request.get_ref();

    let response = proto::ConversionResponse {
      result: input.value,
    };

    Ok(tonic::Response::new(response))
  }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50052".parse().unwrap();

    let seer = SeerService::default();

    let service = tonic_reflection::server::Builder::configure()
      .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
      .build_v1().unwrap();

    let resp = reqwest::get("https://api.binance.com/api/v3/ticker/24hr?symbol=BTCUSDT")
      .await?
      .json::<serde_json::Value>()
      .await?;
    println!("{resp:#?}");

    Server::builder()
      .add_service(service)
      .add_service(SeerServer::new(seer))
      .serve(addr)
      .await.unwrap();

    Ok(())
}
