use std::io::{Read, Seek, Write};
use binrw::{binread, binrw, binwrite, BinRead, BinResult, BinWrite, Endian};
use binrw::error::CustomError;
use binrw::meta::{EndianKind, ReadEndian, WriteEndian};

#[binrw]
#[bw(big)]
#[derive(Debug)]
pub(crate) struct KafkaResponse {
    pub(crate) message_size: i32,
    pub(crate) header: KafkaResponseHeaderV0,
    pub(crate) body: KafkaBody,
}

impl KafkaResponse {
    pub(crate) fn new(message_size: i32, header: KafkaResponseHeaderV0, body: KafkaBody) -> Self {
        Self { message_size, header, body }
    }
}

#[binrw]
#[br(big)]
#[derive(Debug)]
pub(crate) struct KafkaRequest {
    pub(crate) message_size: i32,
    pub(crate) header: KafkaRequestHeaderV2,
    pub(crate) body: KafkaBody,
}

#[binrw]
#[br(big)]
#[derive(Debug)]
pub(crate) struct KafkaRequestHeaderV2 {
    pub(crate) request_api_key: i16,
    pub(crate) request_api_version: i16,
    pub(crate) correlation_id: i32,
    pub(crate) client_id: NullableString,
}

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
            Err(binrw::Error::Custom {
                pos: 0,
                err: Box::new(format!("Invalid length for a nullable string: {length}")),
            })
        } else {
            let mut bytes = Vec::with_capacity(length as usize);
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

    fn write_options<W: Write + Seek>(&self,writer: &mut W, endian: Endian, _: Self::Args<'_>,) -> BinResult<()> {
        match &self.0 {
            None => {
                (-1i16).write_options(writer, endian, ())?;
            }
            Some(s) => {
                let bytes = s.as_bytes();
                if bytes.len() > i16::MAX as usize {
                    return Err(binrw::Error::Custom {
                        pos: 0,
                        err: Box::new("String too long for nullable string"),
                    });
                }

                (bytes.len() as i16).write_options(writer, endian, ())?;

                writer.write_all(bytes)?;
            }
        }
        Ok(())
    }
}

#[binrw]
#[bw(big)]
#[derive(Debug)]
pub(crate) struct KafkaResponseHeaderV0 {
    pub(crate) correlation_id: i32,
}

impl KafkaResponseHeaderV0 {
    pub(crate) fn new(correlation_id: i32) -> Self {
        Self { correlation_id }
    }
}

#[binrw]
#[bw(big)]
#[derive(Debug, Default)]
pub(crate) struct KafkaBody;
