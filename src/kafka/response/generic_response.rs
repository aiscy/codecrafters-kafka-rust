use binrw::{binwrite, BinResult, BinWrite, BinWriterExt, Endian};
use std::fmt::Debug;
use std::io;
use std::io::{Seek, SeekFrom, Write};
use binrw::meta::WriteEndian;

// #[binwrite]
// #[bw(big, stream = s)]
#[derive(Debug)]
pub(crate) struct KafkaGenericResponse<H, B>
where
    H: BinWrite + WriteEndian + Debug, for<'a> H::Args<'a>: Default,
    B: BinWrite + WriteEndian + Debug, for<'a> B::Args<'a>: Default,
{
    // #[bw(try_calc = s.stream_position()?.try_into())]
    // message_size: i32,
    pub(crate) header: H,
    pub(crate) body: B,
}

impl<H, B> KafkaGenericResponse<H, B>
where
    H: BinWrite + WriteEndian + Debug, for<'a> H::Args<'a>: Default,
    B: BinWrite + WriteEndian + Debug, for<'a> B::Args<'a>: Default,
{
    pub fn new(header: H, body: B) -> Self {
        Self { header, body }
    }
}

impl<H, B> BinWrite for KafkaGenericResponse<H, B>
where
    H: BinWrite + WriteEndian + Debug, for<'a> H::Args<'a>: Default,
    B: BinWrite + WriteEndian + Debug, for<'a> B::Args<'a>: Default,
{
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(&self, writer: &mut W, _endian: Endian, _args: Self::Args<'_>) -> BinResult<()> {
        writer.seek_relative(4)?;
        let Self { ref header, ref body } = self;
        header.write(writer)?;
        body.write(writer)?;
        let size = writer.stream_position()? - 4;
        writer.seek(SeekFrom::Start(0))?;
        i32::try_from(size)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
            .write_be(writer)?;
        Ok(())
    }
}