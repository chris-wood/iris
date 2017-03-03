use std::string::String as String;
use std::str;

use common::name as name;
use common::name::Name as Name;
use common::identifier;

use core::packet::typespace;

#[derive(Clone, Debug)]
pub struct Message {
    pub message_bytes: Vec<u8>,
    pub name_offset: usize,
    pub name_segment_offsets: Vec<(usize, usize)>,
    pub key_id_offset: usize,
    pub key_id_length: usize,
    pub content_id_offset: usize,
    pub content_id_length: usize,
    pub name_length: usize,
    pub payload_offset: usize,
    pub payload_length: usize,
}

impl Message {
    pub fn empty() -> Message {
        return Message {
            message_bytes: Vec::new(),
            name_offset: 0,
            name_segment_offsets: Vec::new(),
            name_length: 0,
            key_id_offset: 0,
            key_id_length: 0,
            content_id_offset: 0,
            content_id_length: 0,
            payload_offset: 0,
            payload_length: 0,
        }
    }
    pub fn new(bytes: &[u8]) -> Message {

        let mut byteVector = Vec::new();
        for b in bytes {
            byteVector.push(*b);
        }
        return Message {
            message_bytes: byteVector,
            name_offset: 0,
            name_segment_offsets: Vec::new(),
            name_length: 0,
            key_id_offset: 0,
            key_id_length: 0,
            content_id_offset: 0,
            content_id_length: 0,
            packet_type: typespace::PacketType::Interest,
            payload_offset: 0,
            payload_length: 0,
            validation_offset: 0,
            validation_length: 0,
            validation_type: typespace::ValidationType::Invalid,
            vdd_type: typespace::ValidationDependentDataType::Invalid,
            identifier: identifier::Identifier::empty(),
        }
    }

    pub fn bytes(&self) -> Vec<u8> {
        return self.message_bytes.clone()
    }

    pub fn byte_at(&self, index: usize) -> u8 {
        return self.message_bytes[index];
    }

    pub fn size(&self) -> usize {
        return self.message_bytes.len();
    }

    pub fn get_key_id_overlay(&self) -> Option<(usize, usize)> {
        match (self.key_id_offset, self.key_id_length) {
            (o, l) if o > 0 && l > 0 => Some((o, l)),
            _ => None
        }
    }

    pub fn get_content_id_overlay(&self) -> Option<(usize, usize)> {
        match (self.content_id_offset, self.content_id_length) {
            (o, l) if o > 0 && l > 0 => Some((o, l)),
            _ => None
        }
    }

    pub fn get_name(&self) -> Name {
        let mut name_bytes = Vec::new();

        let mut index = 0;
        let length = self.name_length;
        let offset = self.name_offset;

        while index < length {
            name_bytes.push(self.message_bytes[offset + index]);
            index = index + 1;
        }

        let mut name_segment_offsets = Vec::new();
        let mut offsets = self.name_segment_offsets.clone();
        for (o, l) in offsets {
            name_segment_offsets.push((o - self.name_offset, l));
        }

        return Name::create_from_bytes(&name_bytes, &name_segment_offsets).unwrap();
    }

    // TODO: isControl, isInterest, isContentObject

    pub fn print(self) {
        println!("Packet Details:");
        println!("  packet_type = {}", self.packet_type as usize);
        println!("  name_offset = {}", self.name_offset);
        println!("  name_length = {}", self.name_length);
        println!("  payload_offset = {}", self.payload_offset);
        println!("  payload_length = {}", self.payload_length);
        println!("  validation_offset = {}", self.validation_offset);
        println!("  validation_length = {}", self.validation_length);
    }

    fn decode_tlv_message(msg: &mut message::Message, slice: &[u8], plength: u16, mut offset: usize) -> (usize) {
        let start_offset = offset;

        // The name is mandatory (NOT!)
        let mut next_type: u16 = decoder::read_u16(slice, offset); offset += 2;
        let mut next_length: u16 = decoder::read_u16(slice, offset); offset += 2;
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
        next_type = decoder::read_u16(slice, offset); offset += 2;
        next_length = decoder::read_u16(slice, offset); offset += 2;
        if next_type == 1 {
            msg.payload_offset = offset - 4;
            offset = decode_tlv_payload_value(slice, next_length, offset);
            msg.payload_length = next_length as usize;
        }

        return offset;
    }

    fn decode_tlv_toplevel(msg: &mut message::Message, slice: &[u8], plength: u16, mut offset: usize) -> (usize) {
        while offset < (plength as usize) {
            let top_type: u16 = decoder::read_u16(slice, offset); offset += 2;
            let top_length: u16 = decoder::read_u16(slice, offset); offset += 2;

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
            let name_segment_type: u16 = decoder::read_u16(slice, offset); offset += 2;
            let name_segment_length: u16 = decoder::read_u16(slice, offset); offset += 2;
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
}

#[test]
fn test_get_name() {

}

#[test]
fn test_get_key_id() {

}

#[test]
fn test_get_content_id() {

}
