use std::fmt::Debug;
use std::io::Cursor;
use binrw::{BinRead, BinWrite};
use binrw::meta::WriteEndian;
use bytes::{Buf, BytesMut};
use tokio::io::AsyncWriteExt;
use tokio_util::codec::{Decoder, Encoder};
use log::warn;
use crate::kafka::request::generic_request::KafkaRequest;
use crate::kafka::response::KafkaGenericResponse;

pub(crate) struct KafkaCodec;

impl Decoder for KafkaCodec {
    type Item = KafkaRequest;
    type Error = std::io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let mut cursor = Cursor::new(&src[..]);
        
        match KafkaRequest::read_be(&mut cursor) {
            Ok(request) => {
                let pos = cursor.position() as usize;
                src.advance(pos);
                Ok(Some(request))
            },
            Err(err) => {
                if let binrw::Error::Io(io_err) = &err {
                    if io_err.kind() == std::io::ErrorKind::UnexpectedEof {
                        return Ok(None);
                    }
                }
                warn!("Remaining: {:?}", cursor.into_inner());
                Err(std::io::Error::new(std::io::ErrorKind::Other, err))
            }
        }
    }
}

impl<H, B> Encoder<KafkaGenericResponse<H, B>> for KafkaCodec
where
    H: BinWrite + WriteEndian + Debug,
    for<'a> H::Args<'a>: Default,
    B: BinWrite + WriteEndian + Debug,
    for<'a> B::Args<'a>: Default,
{
    type Error = std::io::Error;

    fn encode(&mut self, item: KafkaGenericResponse<H, B>, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let mut writer = Cursor::new(Vec::with_capacity(128));
        item.write_be(&mut writer).map_err(|err| {
            std::io::Error::new(std::io::ErrorKind::Other, format!("Serialization error: {err:?}"))
        })?;
        dst.extend_from_slice(&writer.into_inner());
        Ok(())
    }
}