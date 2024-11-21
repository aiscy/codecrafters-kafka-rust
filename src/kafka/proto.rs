use std::fmt::Debug;
use std::io::{Read, Seek};
use binrw::{binread, binrw, binwrite, BinRead, BinResult, BinWrite, Endian};
use binrw::io::TakeSeekExt;
use binrw::meta::{EndianKind, ReadEndian};
use crate::kafka::types::{ApiKey, NullableString, TagBuffer};

#[derive(Debug)]
pub(crate) struct KafkaRequest {
    pub(crate) header: KafkaRequestHeader,
    pub(crate) body: KafkaBodyEmpty,
}

impl ReadEndian for KafkaRequest {
    const ENDIAN: EndianKind = EndianKind::Endian(Endian::Big);
}

impl BinRead for KafkaRequest {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<Self> {
        let message_size = i32::read_options(reader, endian, ())?;
        let mut reader = reader.take_seek(message_size.try_into().unwrap());

        let header = KafkaRequestHeader::read_options(&mut reader, endian, ())?;
        let body = KafkaBodyEmpty::read_options(&mut reader, endian, ())?;
        
        /*if reader.limit() != 0 { // TODO uncomment once body is implemented
            Err(binrw::Error::Custom {
                pos: reader.stream_position()?,
                err: Box::new(format!(
                    "unparsed free bytes detected. {} bytes remain after parsing.",
                    reader.limit()
                )),
            })
        } else {
            Ok(Self { header, body })
        }*/

        Ok(Self { header, body })
    }
}


#[binread]
#[br(big)]
#[derive(Debug)]
pub(crate) enum KafkaRequestHeader {
    V2(KafkaRequestHeaderV2),
    V0(KafkaRequestHeaderV1),
    V1(KafkaRequestHeaderV0),
}

#[binread]
#[br(big)]
#[derive(Debug)]
pub(crate) struct  KafkaRequestHeaderV0 {
    pub(crate) request_api_key: ApiKey,
    pub(crate) request_api_version: i16,
    pub(crate) correlation_id: i32,
}

#[binread]
#[br(big)]
#[derive(Debug)]
pub(crate) struct  KafkaRequestHeaderV1 {
    pub(crate) request_api_key: ApiKey,
    pub(crate) request_api_version: i16,
    pub(crate) correlation_id: i32,
    pub(crate) client_id: NullableString,
}

#[binread]
#[br(big)]
#[derive(Debug)]
pub(crate) struct  KafkaRequestHeaderV2 {
    pub(crate) request_api_key: ApiKey,
    pub(crate) request_api_version: i16,
    pub(crate) correlation_id: i32,
    pub(crate) client_id: NullableString,
    _tagged_fields: TagBuffer,
}

#[binwrite]
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

#[binread]
#[br(big)]
#[derive(Debug)]
pub(crate) struct KafkaBodyEmpty;

