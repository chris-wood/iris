use std::string::String as String;
use std::str;

use common::name as name;
use common::name::Name as Name;
use common::identifier;

use core::packet::typespace;
use core::packet::decoder;

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

    // pub fn new(bytes: &[u8]) -> Message {
    //
    //     let mut byteVector = Vec::new();
    //     for b in bytes {
    //         byteVector.push(*b);
    //     }
    //     return Message {
    //         message_bytes: byteVector,
    //         name_offset: 0,
    //         name_segment_offsets: Vec::new(),
    //         name_length: 0,
    //         key_id_offset: 0,
    //         key_id_length: 0,
    //         content_id_offset: 0,
    //         content_id_length: 0,
    //         payload_offset: 0,
    //         payload_length: 0,
    //     }
    // }

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
        println!("Message Details:");
        println!("  name_offset = {}", self.name_offset);
        println!("  name_length = {}", self.name_length);
        println!("  payload_offset = {}", self.payload_offset);
        println!("  payload_length = {}", self.payload_length);
    }

    pub fn new(message_bytes: &[u8]) -> Result<Message, decoder::DecoderError> {
        let mut msg = Message::empty();
        let offset = msg.decode(message_bytes, message_bytes.len(), 0);
        match message_bytes.len() {
            offset =>  Ok(msg),
            _ => Err(decoder::DecoderError::MalformedPacket)
        }
    }

    pub fn decode(slice: &[u8], length: u16, mut offset: usize) -> Result<Message, decoder::DecoderError> {
        let mut msg = Message::empty();

        let start_offset = offset;

        // The name is mandatory (NOT!)
        let mut next_type: u16 = decoder::read_u16(slice, offset); offset += 2;
        let mut next_length: u16 = decoder::read_u16(slice, offset); offset += 2;
        msg.name_length = next_length as usize;
        msg.name_offset = offset;
        if next_type == 0 {
            offset = msg.decode_tlv_name_value(slice, next_length, offset);
        }

        // Check to see if we've reached the end of the packet
        if (start_offset + (length as usize)) == offset {
            return Ok(msg)
        }

        // Check what's next
        next_type = decoder::read_u16(slice, offset); offset += 2;
        next_length = decoder::read_u16(slice, offset); offset += 2;
        if next_type == 1 {
            msg.payload_offset = offset - 4;
            offset = msg.decode_tlv_payload_value(slice, next_length, offset);
            msg.payload_length = next_length as usize;
        }

        return Ok(msg)
    }

    fn decode_tlv_name_value(&mut self, slice: &[u8], plength: u16,  mut offset: usize) -> (usize) {
        let target: usize = (plength as usize) + offset;
        while offset < target {
            let name_segment_type: u16 = decoder::read_u16(slice, offset); offset += 2;
            let name_segment_length: u16 = decoder::read_u16(slice, offset); offset += 2;
            let name_segment_value: &[u8] = &slice[offset .. (offset + name_segment_length as usize)];

            self.name_segment_offsets.push((offset, name_segment_length as usize));
            offset += name_segment_length as usize;
        }

        if target != offset {
            return 0;
        }

        return target;
    }

    fn decode_tlv_payload_value(&mut self, slice: &[u8], plength: u16,  mut offset: usize) -> (usize) {
        let payload_value: &[u8] = &slice[offset .. (offset + plength as usize)];
        return offset + (plength as usize);
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
