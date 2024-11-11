use binrw::{binrw, binwrite, BinRead, BinWrite};

#[binrw]
#[bw(big)]
#[derive(Debug)]
pub struct KafkaResponse {
    pub message_size: i32,
    pub header: KafkaResponseHeaderV0,
    pub body: KafkaBody,
}

impl KafkaResponse {
    pub fn new(message_size: i32, header: KafkaResponseHeaderV0, body: KafkaBody) -> Self {
        Self { message_size, header, body }
    }
}

#[binrw]
#[bw(big)]
#[derive(Debug)]
pub struct KafkaResponseHeaderV0 {
    pub correlation_id: i32,
}

impl KafkaResponseHeaderV0 {
    pub fn new(correlation_id: i32) -> Self {
        Self { correlation_id }
    }
}

#[binrw]
#[bw(big)]
#[derive(Debug, Default)]
pub struct KafkaBody;
