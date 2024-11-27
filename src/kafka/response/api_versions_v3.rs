use crate::kafka::proto::API_REGISTRY;
use crate::kafka::response::common::ApiVersion;
use crate::kafka::types::{CompactArray, ErrorCode, TagBuffer};
use binrw::binwrite;

#[binwrite]
#[bw(big)]
#[derive(Debug)]
pub(crate) struct KafkaResponseApiVersionsV3 {
    pub(crate) error_code: ErrorCode,
    pub(crate) api_versions: CompactArray<ApiVersion>,
    pub(crate) throttle_time_ms: i32,
    _tagged_fields: TagBuffer,
}

impl KafkaResponseApiVersionsV3 {
    pub(crate) fn new(
        error_code: ErrorCode,
        throttle_time_ms: i32,
    ) -> Self {
        let api_versions = API_REGISTRY
            .iter()
            .map(|(api_key, range)|
                ApiVersion::new(
                    *api_key,
                    *range.start(),
                    *range.end(),
                )
            )
            .collect::<Vec<_>>()
            .into();

        Self { error_code, api_versions, throttle_time_ms, _tagged_fields: Default::default() }
    }
}