use binrw::{binwrite, BinWrite, PosValue};
use std::fmt::Debug;
use binrw::meta::WriteEndian;
use crate::kafka::types::helper::pos_marker::PosMarker;

#[binwrite]
#[derive(Debug)]
pub(crate) struct KafkaGenericResponse<H, B>
where
    H: BinWrite + WriteEndian + Debug,
    for<'a> H::Args<'a>: Default,
    B: BinWrite + WriteEndian + Debug,
    for<'a> B::Args<'a>: Default,
{
    message_size: PosMarker<i32>,
    pub(crate) header: H,
    pub(crate) body: B,
    #[bw(write_with = PosMarker::fill, args(message_size))]
    _end_pos: PosValue<()>,
}

impl<H, B> KafkaGenericResponse<H, B>
where
    H: BinWrite + WriteEndian + Debug,
    for<'a> H::Args<'a>: Default,
    B: BinWrite + WriteEndian + Debug,
    for<'a> B::Args<'a>: Default,
{
    pub fn new(header: H, body: B) -> Self {
        Self { message_size: PosMarker::default(), header, body, _end_pos: PosValue { val: (), pos: 0 } }
    }
}
