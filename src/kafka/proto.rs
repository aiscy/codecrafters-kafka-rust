use std::fmt::Debug;
use binrw::{binread, binrw, BinRead, BinWrite};

#[binread]
#[br(big)]
#[derive(Debug)]
pub(crate) struct KafkaBodyEmpty;

