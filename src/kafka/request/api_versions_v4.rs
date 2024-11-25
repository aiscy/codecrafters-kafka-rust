use binrw::{binread};
use crate::kafka::types::{CompactArray, CompactString, ErrorCode, TagBuffer};

#[binread]
#[br(big)]
#[derive(Debug)]
pub(crate) struct KafkaRequestApiVersionsV4 {
    pub(crate) client_software_name: CompactString,
    pub(crate) client_software_version: CompactString,
    _tagged_fields: TagBuffer,
}
