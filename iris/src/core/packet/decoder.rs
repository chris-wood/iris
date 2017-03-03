pub enum DecoderError {
    MalformedPacket,
}

pub fn read_u8(slice: &[u8], offset: usize) -> (u8) {
    return slice[offset] as u8;
}

pub fn read_u16(slice: &[u8], offset: usize) -> (u16) {
    return ((slice[offset] as u16) << 8) | (slice[offset + 1] as u16);
}
