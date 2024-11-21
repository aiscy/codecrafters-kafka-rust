use std::fmt::Debug;
use binrw::{binrw, binwrite, BinRead, BinWrite};
use binrw::meta::{ReadEndian, WriteEndian};
use crate::kafka::types::UnsignedVarInt;

#[binrw]
#[brw(big)]
#[derive(Debug, Clone)]
pub(crate) struct CompactArray<T>
where
    T: BinWrite + BinRead + Debug + Clone + 'static,
    for<'a> <T as BinWrite>::Args<'a>: Clone + Default,
    for<'a> <T as BinRead>::Args<'a>: Clone + Default,
{
    #[bw(calc = if entries.is_empty() { 0.into() } else { (entries.len() as u32 + 1).into() })]
    size: UnsignedVarInt,
    #[br(count = if *size > 0 { *size - 1 } else { 0 })]
    pub(crate) entries: Vec<T>
}

impl<T> CompactArray<T>
where
    T: BinWrite + BinRead + Debug + Clone + 'static,
    for<'a> <T as BinWrite>::Args<'a>: Clone + Default,
    for<'a> <T as BinRead>::Args<'a>: Clone + Default,
{
    pub fn new(entries: Vec<T>) -> Self {
        Self { entries }
    }
}

impl<T> From<Vec<T>> for CompactArray<T>
where
    T: BinWrite + BinRead + Debug + Clone + 'static,
    for<'a> <T as BinWrite>::Args<'a>: Clone + Default,
    for<'a> <T as BinRead>::Args<'a>: Clone + Default,
{
    fn from(value: Vec<T>) -> Self {
        Self::new(value)
    }
}