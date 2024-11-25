use std::cmp::Ordering;
use binrw::meta::{EndianKind, ReadEndian, WriteEndian};
use binrw::{BinRead, BinResult, BinWrite, Endian};
use std::fmt::Debug;
use std::io::{Read, Seek, Write};
use std::num::TryFromIntError;
use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct UnsignedVarInt(pub(crate) u32);

impl UnsignedVarInt {
    pub(crate) fn new(value: u32) -> Self {
        Self(value)
    }
}

impl BinRead for UnsignedVarInt {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(reader: &mut R, _endian: Endian, _args: Self::Args<'_>) -> BinResult<Self> {
        let mut value = 0;
        let mut shift = 0;

        loop {
            let byte = u8::read(reader)?;
            value |= ((byte & 0b0_1111111) as u32) << shift;

            if byte & 0b1_0000000 == 0 {
                break;
            }

            shift += 7;
            if shift >= 32 {
                return Err(binrw::Error::Custom {
                    pos: reader.stream_position()?,
                    err: Box::new("Varint is too long"),
                });
            }
        }

        Ok(Self(value))
    }
}

impl BinWrite for UnsignedVarInt {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(&self, writer: &mut W, _endian: Endian, _args: Self::Args<'_>) -> BinResult<()> {
        let mut value = self.0;

        loop {
            let mut byte = (value & 0b0_1111111) as u8;
            value >>= 7;

            if value != 0 {
                byte |= 0b1_0000000;
            }

            writer.write_all(&[byte])?;

            if value == 0 {
                break;
            }
        }

        Ok(())
    }
}

impl WriteEndian for UnsignedVarInt {
    const ENDIAN: EndianKind = EndianKind::None;
}

impl ReadEndian for UnsignedVarInt {
    const ENDIAN: EndianKind = EndianKind::None;
}

impl Deref for UnsignedVarInt {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<u32> for UnsignedVarInt {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl TryFrom<usize> for UnsignedVarInt {
    type Error = TryFromIntError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let value = u32::try_from(value)?;
        Ok(Self(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use binrw::{BinRead, BinWrite};
    use std::io::Cursor;

    #[test]
    fn test_varint_read_write() {
        let test_cases = [
            (0, vec![0b0_0000000]),
            (1, vec![0b0_0000001]),
            (127, vec![0b0_1111111]),
            (128, vec![0b1_0000000, 0b0_0000001]),
            (255, vec![0b1_1111111, 0b0_0000001]),
            (300, vec![0b1_0101100, 0b0_0000010]),
            (16384, vec![0b1_0000000, 0b1_0000000, 0b0_0000001]),
            (2097151, vec![0b1_1111111, 0b1_1111111, 0b0_1111111]),
            (2097152, vec![0b1_0000000, 0b1_0000000, 0b1_0000000, 0b0_0000001]),
        ];

        for (number, expected_bytes) in test_cases {
            let mut writer = Cursor::new(Vec::new());
            UnsignedVarInt(number).write(&mut writer).unwrap();
            assert_eq!(writer.into_inner(), expected_bytes);

            let mut reader = Cursor::new(expected_bytes);
            let read_value = UnsignedVarInt::read(&mut reader).unwrap();
            assert_eq!(*read_value, number);
        }
    }

    #[test]
    fn test_5bytes_varint() {
        let invalid_bytes = vec![0x80, 0x80, 0x80, 0x80, 0x80];
        let mut reader = Cursor::new(invalid_bytes);
        assert!(UnsignedVarInt::read(&mut reader).is_err());
    }
}