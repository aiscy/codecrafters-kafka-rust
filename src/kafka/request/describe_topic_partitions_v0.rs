use binrw::{binread, binrw};
use crate::kafka::types::{CompactArray, CompactString, TagBuffer};

#[binread]
#[br(big)]
#[derive(Debug)]
pub(crate) struct KafkaRequestDescribeTopicPartitionsV0 {
    pub(crate) topics: CompactArray<TopicRequestV0>,
    pub(crate) response_partition_limit: i32,
    pub(crate) cursor: Option<CursorRequestV0>,
    _tagged_fields: TagBuffer,
}

#[binrw]
#[brw(big)]
#[derive(Debug, Clone)]
pub(crate) struct TopicRequestV0 {
    pub(crate) name: CompactString,
    _tagged_fields: TagBuffer
}

#[binrw]
#[brw(big)]
#[derive(Debug, Clone)]
pub(crate) struct CursorRequestV0 {
    pub(crate) topic_name: CompactString,
    pub(crate) partition_index: i32,
    _tagged_fields: TagBuffer
}