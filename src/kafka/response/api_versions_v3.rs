use binrw::{binrw, binwrite};
use crate::kafka::response::ApiKeyV4;
use crate::kafka::types::{ApiKey, CompactArray, ErrorCode, TagBuffer};

#[binwrite]
#[bw(big)]
#[derive(Debug)]
pub(crate) struct KafkaResponseApiVersionsV3 {
    pub(crate) error_code: ErrorCode,
    pub(crate) api_keys: CompactArray<ApiKeyV3>,
    pub(crate) throttle_time_ms: i32,
    _tagged_fields1: TagBuffer,
}

#[binrw]
#[brw(big)]
#[derive(Debug, Clone)]
pub(crate) struct ApiKeyV3 {
    pub(crate) api_key: ApiKey,
    pub(crate) min_version: i16,
    pub(crate) max_version: i16,
    _tagged_fields: TagBuffer,
}

impl ApiKeyV3 {
    pub fn new(api_key: ApiKey, min_version: i16, max_version: i16) -> Self {
        Self { api_key, min_version, max_version, _tagged_fields: Default::default() }
    }
}

impl KafkaResponseApiVersionsV3 {
    pub(crate) fn new(
        error_code: ErrorCode,
        api_keys: Vec<ApiKeyV3>,
        throttle_time_ms: i32,
    ) -> Self {
        Self { error_code, api_keys: api_keys.into(), throttle_time_ms, _tagged_fields1: Default::default() }
    }
}