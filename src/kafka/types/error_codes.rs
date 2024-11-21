use binrw::binrw;

#[binrw]
#[brw(big, repr = i16)]
#[derive(Debug, Default)]
pub(crate) enum ErrorCode {
    #[default]
    None = 0,
    UnsupportedVersion = 35,
}