use std::io::{Read, Seek, Write};
use std::ops::{Deref, DerefMut};
use binrw::{BinRead, BinResult, BinWrite, Endian};
use crate::kafka::types::UnsignedVarInt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CompactNullableString(pub(crate) Option<String>);

impl BinRead for CompactNullableString {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        _endian: Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<Self> {
        let length = *UnsignedVarInt::read(reader)?;

        if length == 0 {
            return Ok(Self(None));
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

        Ok(Self(Some(string)))
    }
}

impl BinWrite for CompactNullableString {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        _endian: Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<()> {
        match &self.0 {
            None => UnsignedVarInt(0).write(writer),
            Some(string) => {
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
    }
}

impl Deref for CompactNullableString {
    type Target = Option<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CompactNullableString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
