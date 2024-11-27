use binrw::binwrite;

#[binwrite]
#[bw(big)]
#[derive(Debug)]
pub(crate) struct KafkaResponseHeaderV0 {
    pub(crate) correlation_id: i32,
}

impl KafkaResponseHeaderV0 {
    pub(crate) fn new(correlation_id: i32) -> Self {
        Self { correlation_id }
    }
}