use crate::kafka::types::UnsignedVarInt;
use binrw::{BinRead, BinResult, BinWrite, Endian};
use std::io::{Read, Seek, Write};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CompactString(pub(crate) String);

impl BinRead for CompactString {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        _endian: Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<Self> {
        let length = *UnsignedVarInt::read(reader)?;

        if length == 0 {
            return Err(binrw::Error::AssertFail {
                pos: reader.stream_position()?,
                message: "Length must not be zero".into(),
            });
        }

        let length = usize::try_from(length - 1)
            .map_err(|_| binrw::Error::AssertFail {
                pos: reader.stream_position().expect("Should be able to read stream position"),
                message: "Compact string length is too large".to_owned(),
            })?;

        let mut buffer = vec![0; length];
        reader.read_exact(&mut buffer)?;

        let string = String::from_utf8(buffer).map_err(|err| binrw::Error::Custom {
            pos: reader.stream_position().expect("Should be able to read stream position"),
            err: Box::new(err),
        })?;

        Ok(Self(string))
    }
}

impl BinWrite for CompactString {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        _endian: Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<()> {
        let string = &self.0;
        let length = string.len();

        let length_varint = u32::try_from(length + 1).map_err(|_| binrw::Error::AssertFail {
            pos: writer.stream_position().expect("Should be able to read stream position"),
            message: "String length too large".to_owned(),
        })?;

        UnsignedVarInt(length_varint).write(writer)?;

        writer.write_all(string.as_bytes())?;
        Ok(())
    }
}

impl Deref for CompactString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CompactString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
