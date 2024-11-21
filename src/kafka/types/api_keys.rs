use binrw::binrw;

#[binrw]
#[brw(big, repr = i16)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum ApiKey {
    Produce = 0,
    ApiVersions = 18,
    CreateTopics = 19,
}