pub mod decoder;

pub fn decode_packet(slice: &[u8]) {
    decoder::decode_packet_intro(slice, 0);
}
