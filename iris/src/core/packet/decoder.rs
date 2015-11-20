use core::packet::message as message;

// TODO: this should just be decode_tlv
pub fn decode_packet_intro(slice: &[u8], mut offset: usize) -> (message::Message) {

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

    let mut byteVector = Vec::new();
    for b in slice {
        byteVector.push(*b);
    }

    let mut msg = message::Message{
        message_bytes: byteVector,
        packet_type: message::PacketType::Interest,
        name_offset: 0,
        name_length: 0,
        payload_offset: 0,
        payload_length: 0,
        validation_offset: 0,
        validation_length: 0,
        message_name: String::new()
    };

    if msg_type == (message::PacketType::ContentObject as u8) {
        msg.packet_type = message::PacketType::ContentObject;
    }

    // TODO: create the message here
    let mut consumed: usize = decode_tlv_toplevel(&mut msg, slice, plength, offset);

    return msg;
}

fn decode_tlv_parse_one(slice: &[u8], offset: usize) -> (u8) {
    return (slice[offset] as u8);
}

fn decode_tlv_parse_two(slice: &[u8], offset: usize) -> (u16) {
    return ((slice[offset] as u16) << 8) | (slice[offset + 1] as u16);
}

fn decode_tlv_validation_algorithm(msg: &mut message::Message, slice: &[u8], plength: u16, mut offset: usize) -> (usize) {
    return 0;
}

fn decode_tlv_validation_payload(msg: &mut message::Message, slice: &[u8], plength: u16, mut offset: usize) -> (usize) {
    return 0;
}

fn decode_tlv_message(msg: &mut message::Message, slice: &[u8], plength: u16, mut offset: usize) -> (usize) {
    // The name is mandatory
    msg.name_offset = offset;
    let mut next_type: u16 = decode_tlv_parse_two(slice, offset); offset += 2;
    let mut next_length: u16 = decode_tlv_parse_two(slice, offset); offset += 2;
    msg.name_length = next_length as usize;
    println!("TL = {} {}", next_type, next_length);
    if next_type == 0 {
        offset = decode_tlv_name_value(msg, slice, next_length, offset);
    }

    // Check what's next
    next_type = decode_tlv_parse_two(slice, offset); offset += 2;
    next_length = decode_tlv_parse_two(slice, offset); offset += 2;
    println!("TL = {} {}", next_type, next_length);
    if next_type == 1 {
        msg.payload_offset = offset - 4;
        offset = decode_tlv_payload_value(slice, next_length, offset);
        msg.payload_length = next_length as usize;
    }

    return offset;
}

fn decode_tlv_toplevel(msg: &mut message::Message, slice: &[u8], plength: u16, mut offset: usize) -> (usize) {
    while offset < (plength as usize) {
        let top_type: u16 = decode_tlv_parse_two(slice, offset); offset += 2;
        let top_length: u16 = decode_tlv_parse_two(slice, offset); offset += 2;
        println!("top level TL = {} {}", top_type, top_length);

        if top_type == (message::TopLevelType::Interest as u16) {
            println!("interest");
            offset = decode_tlv_message(msg, slice, plength, offset);
        } else if top_type == (message::TopLevelType::ContentObject as u16) {
            println!("data");
            offset = decode_tlv_message(msg, slice, plength, offset);
        } else if top_type == (message::TopLevelType::ValidationAlgorithm as u16) {
            println!("validation alg.");
            offset = decode_tlv_validation_algorithm(msg, slice, plength, offset);
        } else if top_type == (message::TopLevelType::ValidationPayload as u16) {
            println!("validation payload");
            offset = decode_tlv_validation_payload(msg, slice, plength, offset);
        } else {
            // TODO: throw exception!
        }
    }

    return offset;
}

fn decode_tlv_name_value(msg: &mut message::Message, slice: &[u8], plength: u16,  mut offset: usize) -> (usize) {
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

fn decode_tlv_payload_value(slice: &[u8], plength: u16,  mut offset: usize) -> (usize) {
    let payload_value: &[u8] = &slice[offset .. (offset + plength as usize)];

    for b in payload_value {
        print!("{:X} ", b);
    }
    println!("");

    return offset + (plength as usize);
}

fn decode_tlv_validation_payload_value(slice: &[u8], plength: u16,  mut offset: usize) -> (usize) {
    return offset;
}

fn decode_tlv_validation_algorithm_value(slice: &[u8], plength: u16,  mut offset: usize) -> (usize) {
    return offset;
}
