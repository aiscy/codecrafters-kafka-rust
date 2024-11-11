#![allow(unused_imports)]

mod kafka;

use std::io::{Cursor, Write};
use std::net::TcpListener;
use binrw::BinWrite;
use binrw::io::NoSeek;
use bytes::BufMut;
use crate::kafka::proto::KafkaResponseHeaderV0;

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
                let response = kafka::proto::KafkaResponse::new(
                    0, 
                    KafkaResponseHeaderV0::new(7),
                    Default::default()
                );
                let mut writer = NoSeek::new(Vec::with_capacity(8));
                response.write(&mut writer).unwrap();
                match stream.write(&writer.into_inner()) {
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
