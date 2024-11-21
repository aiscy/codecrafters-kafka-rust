mod kafka;

use crate::kafka::proto::{KafkaRequest, KafkaRequestHeader, KafkaResponseHeaderV0};
use binrw::{BinRead, BinWrite};
use std::io::{Cursor, Read, Write};
use std::net::TcpListener;
use crate::kafka::response::{ApiKeyV3, KafkaGenericResponse, KafkaResponseApiVersionsV3};
use crate::kafka::types::{ApiKey, ErrorCode};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                
                let mut buf = [0; 1024];
                stream.read(&mut buf).unwrap();
                let request = KafkaRequest::read_be(&mut Cursor::new(buf)).unwrap();
                let correlation_id = get_correlation_id(&request);
                let request_api_version = get_requests_api_version(&request);
                let request_api_key = get_requests_api_key(&request);
                let response = match request_api_key {
                    ApiKey::ApiVersions => {
                        match request_api_version {
                            0..=4 => {
                                KafkaGenericResponse::new(
                                    KafkaResponseHeaderV0::new(correlation_id),
                                    KafkaResponseApiVersionsV3::new(
                                        ErrorCode::None,
                                        vec![
                                            ApiKeyV3::new(ApiKey::ApiVersions, 0, 4)
                                        ],
                                        250
                                    ))
                            },
                            _ => KafkaGenericResponse::new(
                                KafkaResponseHeaderV0::new(correlation_id),
                                KafkaResponseApiVersionsV3::new(
                                    ErrorCode::UnsupportedVersion,
                                    vec![
                                        ApiKeyV3::new(ApiKey::ApiVersions, 0, 4)
                                    ],
                                    420,
                                )
                            )
                        }
                    },
                    _ => unimplemented!()
                };

                let mut writer = Cursor::new(Vec::with_capacity(64));
                response.write_be(&mut writer).unwrap();
                let response_bytes = &writer.into_inner();
                println!("responding with: {response:?}, bytes: {response_bytes:?}");
                match stream.write(response_bytes) {
                    Ok(size) => { println!("wrote {size} bytes"); }
                    Err(e) => { println!("error: {e}"); }
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
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