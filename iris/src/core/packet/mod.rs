mod decoder;
pub mod message;

pub fn decode_packet(slice: &[u8]) -> (message::Message) {
    return decoder::decode_packet_intro(slice, 0);
}
