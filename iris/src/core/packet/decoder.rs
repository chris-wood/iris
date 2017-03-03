use core::packet::decoder as decoder;
use core::packet::message as message;
use core::packet::typespace as typespace;
use common::identifier;

pub enum DecoderError {
    MalformedPacket,
}

// TODO(cawood): rename these to "read_word" and "read_byte"

pub fn decode_tlv_parse_one(slice: &[u8], offset: usize) -> (u8) {
    return slice[offset] as u8;
}

pub fn decode_tlv_parse_two(slice: &[u8], offset: usize) -> (u16) {
    return ((slice[offset] as u16) << 8) | (slice[offset + 1] as u16);
}

pub fn decode_packet_intro(slice: &[u8], mut offset: usize) -> Result<message::Message, DecoderError> {

    // Decode the fixed header
    let version: u8 = decode_tlv_parse_one(slice, offset); offset += 1;
    let msg_type: u8 = decode_tlv_parse_one(slice, offset); offset += 1;
    let mut plength: u16 = decode_tlv_parse_two(slice, offset); offset += 2;
    plength = plength - 8; // packet length includes the fixed header
    let rsvd: u16 = decode_tlv_parse_two(slice, offset); offset += 2;
    let flags: u8 = decode_tlv_parse_one(slice, offset); offset += 1;
    let header_length: u8 = decode_tlv_parse_one(slice, offset); offset += 1;

    // Sanity check on the length
    if slice.len() != (offset + plength as usize) {
        return Err(DecoderError::MalformedPacket)
    }

    let mut byteVector = Vec::new();
    for b in slice {
        byteVector.push(*b);
    }

    let mut msg = message::Message{
        message_bytes: byteVector,
        packet_type: typespace::PacketType::Interest,
        name_offset: 0,
        name_segment_offsets: Vec::new(),
        name_length: 0,
        key_id_offset: 0,
        key_id_length: 0,
        content_id_offset: 0,
        content_id_length: 0,
        payload_offset: 0,
        payload_length: 0,
        validation_offset: 0,
        validation_length: 0,
        validation_type: typespace::ValidationType::Invalid,
        vdd_type: typespace::ValidationDependentDataType::Invalid,
        identifier: identifier::Identifier::empty(),
    };

    if msg_type == (typespace::PacketType::ContentObject as u8) {
        msg.packet_type = typespace::PacketType::ContentObject;
    }

    let consumed: usize = decode_tlv_toplevel(&mut msg, slice, plength, offset);
    if consumed != slice.len() {
        return Err(DecoderError::MalformedPacket)
    }

    return Ok(msg);
}

// fn decode_tlv_validation_dependent_data(msg: &mut message::Message, slice: &[u8], plength: u16, mut offset: usize) -> (usize) {
//     let start_offset = offset;
//
//     // Parse the validation dependent data
//     let mut vdd_type: u16 = decode_tlv_parse_two(slice, offset); offset += 2;
//     let mut vdd_length: u16 = decode_tlv_parse_two(slice, offset); offset += 2;
//     msg.vdd_type = typespace::ParseValidationDependentDataType(vdd_type);
//
//     // XXX parse out the type of validator
//     match msg.vdd_type {
//         message::ValidationDependentDataType::KeyId => println!("woo"),
//         message::ValidationDependentDataType::PublicKey => println!("woo"),
//         message::ValidationDependentDataType::Certificate => println!("woo"),
//         message::ValidationDependentDataType::KeyName => println!("woo"),
//         message::ValidationDependentDataType::SignatureTime => println!("woo"),
//         message::ValidationDependentDataType::Invalid => return 0
//     };
//
//     return offset;
// }

fn decode_tlv_validation_algorithm(msg: &mut message::Message, slice: &[u8], plength: u16, mut offset: usize) -> (usize) {
    let start_offset = offset;

    // Parse the validation type
    let mut val_type: u16 = decode_tlv_parse_two(slice, offset); offset += 2;
    let mut val_length: u16 = decode_tlv_parse_two(slice, offset); offset += 2;
    msg.validation_offset = start_offset;
    msg.validation_length = val_length as usize;
    msg.validation_type = typespace::ParseValidationType(val_type);

    // Parse the validation dependent data
    // XXX: decode the Validation algorithm

    return offset;
}

fn decode_tlv_validation_payload(msg: &mut message::Message, slice: &[u8], plength: u16, mut offset: usize) -> (usize) {
    return 0;
}

fn decode_tlv_message(msg: &mut message::Message, slice: &[u8], plength: u16, mut offset: usize) -> (usize) {
    let start_offset = offset;

    // The name is mandatory (NOT!)
    let mut next_type: u16 = decode_tlv_parse_two(slice, offset); offset += 2;
    let mut next_length: u16 = decode_tlv_parse_two(slice, offset); offset += 2;
    msg.name_length = next_length as usize;
    msg.name_offset = offset;
    if next_type == 0 {
        offset = decode_tlv_name_value(msg, slice, next_length, offset);
    }

    // Check to see if we've reached the end of the packet
    if (start_offset + (plength as usize)) == offset {
        return offset;
    }

    // Check what's next
    next_type = decode_tlv_parse_two(slice, offset); offset += 2;
    next_length = decode_tlv_parse_two(slice, offset); offset += 2;
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

        if top_type == (typespace::TopLevelType::Interest as u16) {
            offset = decode_tlv_message(msg, slice, top_length, offset);
        } else if top_type == (typespace::TopLevelType::ContentObject as u16) {
            offset = decode_tlv_message(msg, slice, top_length, offset);
        } else if top_type == (typespace::TopLevelType::ValidationAlgorithm as u16) {
            offset = decode_tlv_validation_algorithm(msg, slice, top_length, offset);
        } else if top_type == (typespace::TopLevelType::ValidationPayload as u16) {
            offset = decode_tlv_validation_payload(msg, slice, top_length, offset);
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

        msg.name_segment_offsets.push((offset, name_segment_length as usize));
        offset += name_segment_length as usize;
    }

    if target != offset {
        return 0;
    }

    return target;
}

fn decode_tlv_payload_value(slice: &[u8], plength: u16,  mut offset: usize) -> (usize) {
    let payload_value: &[u8] = &slice[offset .. (offset + plength as usize)];
    return offset + (plength as usize);
}

fn decode_tlv_validation_payload_value(slice: &[u8], plength: u16,  mut offset: usize) -> (usize) {
    return offset;
}

fn decode_tlv_validation_algorithm_value(slice: &[u8], plength: u16,  mut offset: usize) -> (usize) {
    return offset;
}
