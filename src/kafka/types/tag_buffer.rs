use binrw::binrw;

#[binrw]
#[brw(big, magic = b"\x00")]
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct TagBuffer; // TODO let's assume they are empty for now