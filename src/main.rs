mod kafka;

use crate::kafka::codec::KafkaCodec;
use crate::kafka::proto::ApiVersionsResponse;
use crate::kafka::request::generic_request::{KafkaRequest, KafkaRequestHeader};
use crate::kafka::response::KafkaResponseHeaderV0;
use crate::kafka::response::{KafkaGenericResponse};
use crate::kafka::types::ApiKey;
use futures::SinkExt;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio_stream::StreamExt;
use tokio_util::codec::Framed;
use tracing::level_filters::LevelFilter;
use tracing::{error, info, instrument};
use tracing_subscriber::EnvFilter;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy()
        )
        .init();

    let listener = TcpListener::bind("127.0.0.1:9092").await?;
    info!("Listening on: {}", listener.local_addr()?);

    loop {
        let (socket, addr) = listener.accept().await?;
        info!(client = %addr, "Accepted new connection");
        tokio::spawn(handle_client(socket, addr));
    }

    Ok(())
}

#[instrument(skip(socket))]
async fn handle_client(socket: TcpStream, addr: SocketAddr) {
    let mut framed = Framed::new(socket, KafkaCodec);
    info!(client = %addr, "Client handler spawned");

    while let Some(request) = framed.next().await {
        match request {
            Ok(req) => {
                info!(client = %addr, request = ?req, "Received request");

                let correlation_id = get_correlation_id(&req);
                let api_version = get_requests_api_version(&req);
                let api_key = get_requests_api_key(&req);
                let response = match api_key {
                    ApiKey::ApiVersions => {
                        KafkaGenericResponse::new(
                            KafkaResponseHeaderV0::new(correlation_id),
                            ApiVersionsResponse::new(api_version)
                        )
                    },
                    _ => {
                        error!(client = %addr, api_key = ?api_key, "Unhandled API key");
                        continue;
                    }
                };
                if let Err(err) = framed.send(response).await {
                    error!(client = %addr, error = %err, "Failed to send response");
                }
            }
            Err(err) => {
                error!(client = %addr, error = %err, "Error decoding request");
                break;
            }
        }
    }

    info!(client = %addr, "Connection closed");
}

fn get_requests_api_version(request: &KafkaRequest) -> i16 {
    match &request.header {
        KafkaRequestHeader::V2(header) => header.request_api_version,
        KafkaRequestHeader::V1(header) => header.request_api_version,
        KafkaRequestHeader::V0(header) => header.request_api_version,
    }
}

fn get_requests_api_key(request: &KafkaRequest) -> &ApiKey {
    match &request.header {
        KafkaRequestHeader::V2(header) => &header.request_api_key,
        KafkaRequestHeader::V1(header) => &header.request_api_key,
        KafkaRequestHeader::V0(header) => &header.request_api_key,
    }
}

fn get_correlation_id(request: &KafkaRequest) -> i32 {
    match &request.header {
        KafkaRequestHeader::V2(header) => header.correlation_id,
        KafkaRequestHeader::V1(header) => header.correlation_id,
        KafkaRequestHeader::V0(header) => header.correlation_id,
    }
}