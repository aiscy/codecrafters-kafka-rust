use bytes::BytesMut;
use tokio_util::codec::Encoder;
use crate::kafka::proto::KafkaResponse;

struct KafkaCodec;

// impl Encoder<KafkaResponse> for KafkaCodec {
//     type Error = binrw::io::Error;
// 
//     fn encode(&mut self, item: KafkaResponse, dst: &mut BytesMut) -> Result<(), Self::Error> {
//         todo!()
//     }
// }