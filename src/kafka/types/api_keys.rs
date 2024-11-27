use binrw::binrw;

#[binrw]
#[brw(big, repr = i16)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum ApiKey {
    Produce = 0,
    ApiVersions = 18,
    CreateTopics = 19,
    DescribeTopicPartitions = 75,
}