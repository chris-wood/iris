mod decoder;
pub mod message;

use common;

pub fn decode_packet(slice: &[u8]) -> (message::Message) {
    return decoder::decode_packet_intro(slice, 0);
}

pub fn prepend_tlv(buffer: &[u8], tlv_type: u8, tlv_length: u8, tlv_value: &[u8]) {
    // TODO
}
