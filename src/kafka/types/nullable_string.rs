use binrw::{BinRead, BinResult, BinWrite, Endian};
use std::io::{Read, Seek, Write};
use std::ops::Deref;
use binrw::meta::{EndianKind, ReadEndian, WriteEndian};

#[derive(Debug, Clone, PartialEq, Default)]
pub(crate) struct NullableString(pub(crate) Option<String>);

impl ReadEndian for NullableString {
    const ENDIAN: EndianKind = EndianKind::Endian(Endian::Big);
}

impl BinRead for NullableString {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(reader: &mut R, endian: Endian, _args: Self::Args<'_>) -> BinResult<Self> {
        let length: i16 = i16::read_options(reader, endian, ())?;

        if length == -1 {
            Ok(Self(None))
        } else if length < -1 {
            Err(binrw::Error::AssertFail {
                pos: 0,
                message: format!("Invalid length for a nullable string: {length}"),
            })
        } else {
            let mut bytes = vec![0u8; length as usize];
            reader.read_exact(&mut bytes)?;

            match String::from_utf8(bytes) {
                Ok(s) => Ok(Self(Some(s))),
                Err(e) => Err(binrw::Error::Custom {
                    pos: 0,
                    err: Box::new(e),
                }),
            }
        }
    }
}

impl WriteEndian for NullableString {
    const ENDIAN: EndianKind = EndianKind::Endian(Endian::Big);
}

impl BinWrite for NullableString {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(&self, writer: &mut W, endian: Endian, _: Self::Args<'_>) -> BinResult<()> {
        match &self.0 {
            None => {
                (-1i16).write_options(writer, endian, ())?;
            }
            Some(s) => {
                let bytes = s.as_bytes();
                if bytes.len() > i16::MAX as usize {
                    return Err(binrw::Error::Custom {
                        pos: 0,
                        err: Box::new(format!("String is too long for a nullable string: {}", bytes.len())),
                    });
                }

                (bytes.len() as i16).write_options(writer, endian, ())?;

                writer.write_all(bytes)?;
            }
        }
        Ok(())
    }
}

impl Deref for NullableString {
    type Target = Option<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Option<String>> for NullableString {
    fn from(value: Option<String>) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_empty_string() {
        let null_string = NullableString(None);
        let mut buffer = Vec::new();
        let mut cursor = Cursor::new(&mut buffer);
        null_string.write(&mut cursor).unwrap();
        
        cursor.set_position(0);
        let read_null: NullableString = BinRead::read(&mut cursor).unwrap();
        assert_eq!(null_string, read_null);
    }
    
    #[test]
    fn test_non_empty_string() {
        let test_string = NullableString(Some("hello".to_owned()));
        let mut buffer = Vec::new();
        let mut cursor = Cursor::new(&mut buffer);
        test_string.write(&mut cursor).unwrap();
        
        cursor.set_position(0);
        let read_string = NullableString::read(&mut cursor).unwrap();
        assert_eq!(test_string, read_string);
    }
}