use std::io::{Read, Seek, Write};
use std::marker::PhantomData;
use binrw::{binread, binrw, binwrite, BinRead, BinResult, BinWrite, Endian};
use binrw::error::CustomError;
use binrw::meta::{EndianKind, ReadEndian, WriteEndian};
use crate::kafka::types::nullable_string::NullableString;
use crate::kafka::misc::Counter;

#[binwrite]
#[bw(big, stream = c, map_stream = Counter::new)]
#[derive(Debug)]
pub(crate) struct KafkaResponse<H, B>
where
    for<'a> H: BinWrite<Args<'a> = ()>,
    for<'a> B: BinWrite<Args<'a> = ()>,
{
    #[bw(calc = c.size() as i32)]
    _message_size: i32,
    pub(crate) header: H,
    pub(crate) body: B,
}

impl<H, B> KafkaResponse<H, B>
where
        for<'a> H: BinWrite<Args<'a>=()>,
        for<'a> B: BinWrite<Args<'a>=()>,
{
    pub fn new(header: H, body: B) -> Self {
        Self { header, body }
    }
}

#[binread]
#[br(big)]
#[derive(Debug)]
pub(crate) struct KafkaRequest {
    pub(crate) message_size: i32,
    pub(crate) header: KafkaRequestHeaderV2,
    pub(crate) body: KafkaBodyDummy,
}

#[binread]
#[br(big)]
#[derive(Debug)]
pub(crate) struct KafkaRequestHeaderV2 {
    pub(crate) request_api_key: ApiKey,
    pub(crate) request_api_version: i16,
    pub(crate) correlation_id: i32,
    pub(crate) client_id: NullableString,
}

#[binrw]
#[brw(big, repr = i16)]
#[derive(Debug)]
pub(crate) enum ApiKey {
    Produce = 0,
    ApiVersions = 18,
    CreateTopics = 19,
}

#[binrw]
#[brw(big, repr = i16)]
#[derive(Debug)]
pub(crate) enum ErrorCode {
    None = 0,
    UnsupportedVersion = 35,
}

#[binwrite]
#[bw(big)]
#[derive(Debug)]
pub(crate) struct KafkaResponseHeaderV0 {
    pub(crate) correlation_id: i32,
}

#[binwrite]
#[bw(big)]
#[derive(Debug)]
pub(crate) struct KafkaResponseApiVersionsV4 {
    pub(crate) correlation_id: i32,
    pub(crate) error_code: ErrorCode,
}

impl KafkaResponseApiVersionsV4 {
    pub(crate) fn new(correlation_id: i32, error_code: ErrorCode) -> Self {
        Self { correlation_id, error_code }
    }
}

impl KafkaResponseHeaderV0 {
    pub(crate) fn new(correlation_id: i32) -> Self {
        Self { correlation_id }
    }
}

#[binrw]
#[brw(big)]
#[derive(Debug, Default)]
pub(crate) struct KafkaBodyDummy;
