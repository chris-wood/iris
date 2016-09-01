mod decoder;
pub mod message;

use common;

pub fn decode_packet(slice: &[u8]) -> Result<message::Message, decoder::DecoderError> {
    return decoder::decode_packet_intro(slice, 0);
}
