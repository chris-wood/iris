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
    println!("      {}", rsvd);
    if msg_type == 0 {
        decode_tlv_interest(slice, plength, offset);
    } else if msg_type == 1 {
        decode_tlv_content_object(slice, plength, offset);
    }
}

fn decode_tlv_parse_one(slice: &[u8], offset: usize) -> (u8) {
    return (slice[offset] as u8);
}

fn decode_tlv_parse_two(slice: &[u8], offset: usize) -> (u16) {
    return ((slice[offset] as u16) << 8) | (slice[offset + 1] as u16);
}

fn decode_tlv_interest(slice: &[u8], plength: u16, mut offset: usize) {
    println!("decoding an interest");
}

fn decode_tlv_content_object(slice: &[u8], plength: u16, mut offset: usize) {
    println!("decoding a content object");
    let mut consumed: usize = decode_tlv_message(slice, plength, offset);
}

fn decode_tlv_message(slice: &[u8], plength: u16, mut offset: usize) -> (usize) {
    let top_type: u16 = decode_tlv_parse_two(slice, offset); offset += 2;
    let top_length: u16 = decode_tlv_parse_two(slice, offset); offset += 2;

    println!("top level TL = {} {}", top_type, top_length);

    // TODO: replace these magic numbers with constants
    // TODO: do something different for each type?
    if top_type == 1 {

    } else if top_type == 2 {

    }

    // The name is mandatory
    let mut next_type: u16 = decode_tlv_parse_two(slice, offset); offset += 2;
    let mut next_length: u16 = decode_tlv_parse_two(slice, offset); offset += 2;
    println!("TL = {} {}", next_type, next_length);
    if next_type == 0 {
        offset = decode_tlv_name_value(slice, next_length, offset);
    }

    // Check what's next
    next_type = decode_tlv_parse_two(slice, offset); offset += 2;
    next_length = decode_tlv_parse_two(slice, offset); offset += 2;
    println!("TL = {} {}", next_type, next_length);
    if next_type == 1 {
        offset = decode_tlv_payload_value(slice, next_length, offset);
    }

    return offset;
}

fn decode_tlv_payload_value(slice: &[u8], plength: u16,  mut offset: usize) -> (usize) {
    let payload_value: &[u8] = &slice[offset .. (offset + plength as usize)];

    for b in payload_value {
        print!("{:X} ", b);
    }
    println!("");

    return offset;
}

fn decode_tlv_name_value(slice: &[u8], plength: u16,  mut offset: usize) -> (usize) {
    let target: usize = (plength as usize) + offset;
    while offset < target {
        let name_segment_type: u16 = decode_tlv_parse_two(slice, offset); offset += 2;
        let name_segment_length: u16 = decode_tlv_parse_two(slice, offset); offset += 2;
        let name_segment_value: &[u8] = &slice[offset .. (offset + name_segment_length as usize)];
        offset += name_segment_length as usize;

        print!("NAME TLV {} {} ", name_segment_type, name_segment_length);
        for b in name_segment_value {
            print!("{:X} ", b);
        }
        println!("");
    }

    return offset;
}
