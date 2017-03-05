mod decoder;
mod packet;
mod validation;
pub mod message;
pub mod typespace;

pub fn decode(slice: &[u8]) -> Result<packet::Packet, decoder::DecoderError> {
    return packet::Packet::decode(slice, 0);
}
