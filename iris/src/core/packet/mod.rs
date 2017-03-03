mod decoder;
mod packet;
pub mod message;
pub mod typespace;

pub fn decode(slice: &[u8]) -> Result<packet::Packet, decoder::DecoderError> {
    return packet::decode_packet(slice, 0);
}
