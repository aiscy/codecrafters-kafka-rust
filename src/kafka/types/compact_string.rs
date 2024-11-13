use std::io::{Read, Seek};
use binrw::{BinRead, BinResult, BinWrite, Endian};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CompactString(String);
