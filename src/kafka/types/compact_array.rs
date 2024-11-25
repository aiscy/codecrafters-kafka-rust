use std::fmt::Debug;
use binrw::{binrw, BinRead, BinWrite};
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
    #[bw(try_calc = match entries {
        None => Ok(0.into()), Some(e) => (e.len() + 1).try_into()
    })]
    size: UnsignedVarInt,
    #[br(if(*size > 0), count = if *size > 0 { *size - 1 } else { 0 })]
    pub(crate) entries: Option<Vec<T>>,
}

impl<T> CompactArray<T>
where
    T: BinWrite + BinRead + Debug + Clone + 'static,
    for<'a> <T as BinWrite>::Args<'a>: Clone + Default,
    for<'a> <T as BinRead>::Args<'a>: Clone + Default,
{
    pub(crate) fn new(entries: Option<Vec<T>>) -> Self {
        Self { entries }
    }
}

impl<T> From<Option<Vec<T>>> for CompactArray<T>
where
    T: BinWrite + BinRead + Debug + Clone + 'static,
    for<'a> <T as BinWrite>::Args<'a>: Clone + Default,
    for<'a> <T as BinRead>::Args<'a>: Clone + Default,
{
    fn from(value: Option<Vec<T>>) -> Self {
        Self::new(value)
    }
}

impl<T> From<Vec<T>> for CompactArray<T>
where
    T: BinWrite + BinRead + Debug + Clone + 'static,
    for<'a> <T as BinWrite>::Args<'a>: Clone + Default,
    for<'a> <T as BinRead>::Args<'a>: Clone + Default,
{
    fn from(value: Vec<T>) -> Self {
        Self::new(Some(value))
    }
}
