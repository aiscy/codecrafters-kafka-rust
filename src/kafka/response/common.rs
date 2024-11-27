use binrw::binrw;
use crate::kafka::types::{ApiKey, TagBuffer};

#[binrw]
#[brw(big)]
#[derive(Debug, Clone)]
pub(crate) struct ApiVersion {
    pub(crate) api_key: ApiKey,
    pub(crate) min_version: i16,
    pub(crate) max_version: i16,
    _tagged_fields: TagBuffer,
}

impl ApiVersion {
    pub(crate) fn new(api_key: ApiKey, min_version: i16, max_version: i16) -> Self {
        Self { api_key, min_version, max_version, _tagged_fields: Default::default() }
    }
}