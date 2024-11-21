use binrw::{binrw, binwrite};
use crate::kafka::types::{ApiKey, CompactArray, ErrorCode, TagBuffer};

#[binwrite]
#[bw(big)]
#[derive(Debug)]
pub(crate) struct KafkaResponseApiVersionsV4 {
    pub(crate) error_code: ErrorCode,
    pub(crate) api_keys: CompactArray<ApiKeyV4>,
    pub(crate) throttle_time_ms: i32,
    _tagged_fields: TagBuffer,
}

#[binrw]
#[brw(big)]
#[derive(Debug, Clone)]
pub(crate) struct ApiKeyV4 {
    pub(crate) api_key: ApiKey,
    pub(crate) min_version: i16,
    pub(crate) max_version: i16,
    _tagged_fields: TagBuffer,
}

impl ApiKeyV4 {
    pub fn new(api_key: ApiKey, min_version: i16, max_version: i16) -> Self {
        Self { api_key, min_version, max_version, _tagged_fields: Default::default() }
    }
}

impl KafkaResponseApiVersionsV4 {
    pub(crate) fn new(
        error_code: ErrorCode,
        api_keys: Vec<ApiKeyV4>,
        throttle_time_ms: i32,
    ) -> Self {
        Self { error_code, api_keys: api_keys.into(), throttle_time_ms, _tagged_fields: Default::default() }
    }
}