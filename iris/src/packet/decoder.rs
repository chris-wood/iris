// TODO: this should just be decode_tlv
pub fn decode_packet_intro(slice: &[u8], mut offset: usize) {

    // Decode the fixed header
    let version: u8 = decode_tlv_parse_one(slice, offset); offset += 1;
    let msg_type: u8 = decode_tlv_parse_one(slice, offset); offset += 1;
    let mut plength: u16 = decode_tlv_parse_two(slice, offset); offset += 2;
    plength = plength - 8; // packet length includes the fixed header
    let rsvd: u16 = decode_tlv_parse_two(slice, offset); offset += 2;
    let flags: u8 = decode_tlv_parse_one(slice, offset); offset += 1;
    let header_length: u8 = decode_tlv_parse_one(slice, offset); offset += 1;

    // Debug header info
    println!("TLV = {} {} {}", version, msg_type, plength);
    if msg_type == 0 {
        println!("interest time...");
        decode_tlv_interest(plength, &slice[offset .. (offset + plength as usize)]);
    } else if msg_type == 1 {
        decode_tlv_content_object(plength, &slice[offset .. (offset + plength as usize)]);
    }
}

fn decode_tlv_parse_one(slice: &[u8], offset: usize) -> (u8) {
    return (slice[offset] as u8);
}

fn decode_tlv_parse_two(slice: &[u8], offset: usize) -> (u16) {
    return ((slice[offset] as u16) << 8) | (slice[offset + 1] as u16);
}

fn decode_tlv_interest(plength: u16, slice: &[u8]) {
    // TODO
    println!("decoding an interest");
}

fn decode_tlv_content_object(plength: u16, slice: &[u8]) {
    // TODO
    println!("decoding a content object");
    let mut consumed: usize = decode_tlv_message(plength, slice);
}

fn decode_tlv_message(plength: u16, slice: &[u8]) -> (usize) {
    let mut offset: usize = 0;
    let message_type: u16 = decode_tlv_parse_two(slice, offset); offset += 2;
    let message_length: u16 = decode_tlv_parse_two(slice, offset); offset += 2;

    println!("message TL = {} {}", message_type, message_length);

    let mut consumed: usize = offset; // number of bytes consumed

    // TODO: replace these magic numbers with constants
    if message_type == 0 {

    } else if message_type == 1 {
        decode_tlv_payload(message_length, &slice[consumed .. (consumed + message_length as usize)]);
    }

    return consumed;
}

fn decode_tlv_payload(plength: u16, slice: &[u8]) -> (usize) {
    return 0;
}

fn decode_tlv_name(plength: u16, slice: &[u8]) -> (usize) {
    return 0;
}
