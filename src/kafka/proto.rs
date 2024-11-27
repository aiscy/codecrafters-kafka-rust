use crate::kafka::response::{KafkaResponseApiVersionsV3, KafkaResponseApiVersionsV4};
use crate::kafka::types::{ApiKey, ErrorCode};
use binrw::meta::{EndianKind, WriteEndian};
use binrw::Endian::Big;
use binrw::{BinWrite, Endian};
use std::collections::HashMap;
use std::io::{Seek, Write};
use std::ops::RangeInclusive;
use std::sync::LazyLock;

pub(crate) static API_REGISTRY: LazyLock<HashMap<ApiKey, RangeInclusive<i16>>> = LazyLock::new(|| {
    use crate::kafka::types::ApiKey::*;
    let mut registry = HashMap::new();
    registry.insert(ApiVersions, 0..=4);
    registry.insert(DescribeTopicPartitions, 0..=0);
    registry
});

#[derive(Debug)]
pub(crate) enum ApiVersionsResponse {
    V3(KafkaResponseApiVersionsV3),
    V4(KafkaResponseApiVersionsV4),
}

impl ApiVersionsResponse {
    pub(crate) fn new(version: i16) -> Self {
        match version {
            3 => ApiVersionsResponse::V3(KafkaResponseApiVersionsV3::new(ErrorCode::None, 0)),
            4 => ApiVersionsResponse::V4(KafkaResponseApiVersionsV4::new(ErrorCode::None, 0)),
            _ => ApiVersionsResponse::V4(KafkaResponseApiVersionsV4::new(ErrorCode::UnsupportedVersion, 0)),
        }
    }
}

impl BinWrite for ApiVersionsResponse {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        options: Endian,
        _: Self::Args<'_>,
    ) -> binrw::BinResult<()> {
        match self {
            ApiVersionsResponse::V3(resp) => resp.write_be(writer),
            ApiVersionsResponse::V4(resp) => resp.write_be(writer),
        }
    }
}

impl WriteEndian for ApiVersionsResponse {
    const ENDIAN: EndianKind = EndianKind::Endian(Big);
}
