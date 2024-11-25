use std::cell::Cell;
use binrw::{BinResult, BinWrite, Endian};
use std::io::{Seek, SeekFrom, Write};

#[derive(Debug)]
pub(crate) struct PosMarker<T> {
    pos: Cell<u64>,
    value: T,
}

impl<T> PosMarker<T>
where
    T: BinWrite<Args<'static>=()> + TryFrom<u64>,
    T::Error: binrw::error::CustomError + 'static,
{
    #[binrw::writer(writer, endian)]
    pub(crate) fn fill<U>(_: &U, this: &Self) -> BinResult<()> {
        let pos = writer.stream_position()?;

        let size = if let Some(size) = pos.checked_sub(size_of::<T>() as u64) {
            size
        } else {
            return Err(binrw::error::Error::Custom {
                pos,
                err: Box::new("Position underflow"),
            });
        };

        let value = T::try_from(size)
            .map_err(|err| binrw::error::Error::Custom {
                pos,
                err: Box::new(err),
            })?;

        writer.seek(SeekFrom::Start(this.pos.get()))?;
        value.write_options(writer, endian, ())
    }
}

impl<T> BinWrite for PosMarker<T>
where
    T: BinWrite<Args<'static>=()> + Default,
{
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        args: Self::Args<'_>,
    ) -> BinResult<()> {
        self.pos.set(writer.stream_position()?);
        T::default().write_options(writer, endian, args)
    }
}

impl Default for PosMarker<i32> {
    fn default() -> Self {
        Self { pos: Cell::new(0), value: 0 }
    }
}