use std::string::String as String;
use std::str;

use common::name as name;
use common::name::Name as Name;

#[derive(PartialEq, Clone, Debug)]
pub enum TopLevelType {
    Interest = 0x0001,
    ContentObject = 0x0002,
    ValidationAlgorithm = 0x0003,
    ValidationPayload = 0x0004
}

#[derive(PartialEq, Clone, Debug)]
pub enum PacketType {
    Interest = 0x0000,
    ContentObject = 0x0001,
    InterestReturn = 0x0002
}

#[derive(PartialEq, Clone, Debug)]
enum HopByHopHeaderType {
    InterestLifetime = 0x0001,
    CacheTime = 0x0002
}

#[derive(PartialEq, Clone, Debug)]
enum MessageType {
    Name = 0x0000,
    Payload = 0x0001
}

#[derive(PartialEq, Clone, Debug)]
enum NameType {
    NameSegment = 0x0001,
    PayloadID = 0x0002,
    AppLower = 0x1000,
    AppUpper = 0x1FFF
}

#[derive(PartialEq, Clone, Debug)]
enum InterestMessageTLVType {
    KeyIdRestriction = 0x0002,
    HashRestriction = 0x0003
}

#[derive(PartialEq, Clone, Debug)]
enum ContentObjectMessageTLVType {
    PayloadType = 0x0005,
    ExpiryTime = 0x0006
}

#[derive(PartialEq, Clone, Debug)]
enum ValidationType {
    Crc32 = 0x0002,
    HmacSha256 = 0x0004,
    Vmac128 = 0x0005,
    RsaSha256 = 0x0006,
    EcSecp256K1 = 0x0007,
    EcSecp384R1 = 0x0008,
}

#[derive(PartialEq, Clone, Debug)]
enum ValidationDependentDataType {
    KeyId = 0x0009,
    PublicKey = 0x000B,
    Certificate = 0x000C,
    KeyName = 0x000E,
    SignatureTime = 0x000F
}

#[derive(Clone, Debug)]
pub struct Message {
    pub message_bytes: Vec<u8>,
    pub packet_type: PacketType,
    pub name_offset: usize,
    pub name_segment_offsets: Vec<(usize, usize)>,
    pub key_id_offset: usize,
    pub key_id_length: usize,
    pub content_id_offset: usize,
    pub content_id_length: usize,
    pub name_length: usize,
    pub payload_offset: usize,
    pub payload_length: usize,
    pub validation_offset: usize,
    pub validation_length: usize,

    pub message_name: String
}

impl Message {
    // A public constructor
    // pub fn new(bytes: &[u8]) -> Message {
    //     let mut byteVector = Vec::new();
    //     for b in bytes {
    //         byteVector.push(*b);
    //     }
    //     return Message { message_bytes: byteVector };
    // }

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
            packet_type: PacketType::Interest,
            payload_offset: 0, //new_payload_offset,
            payload_length: 0, //new_payload_length,
            validation_offset: 0, //new_validation_offset,
            validation_length: 0, //new_validation_length,
            message_name: String::new()
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

        // let num_components = self.name_segment_offsets.len();
        let mut name_segment_offsets = Vec::new(); // =
        let mut offsets = self.name_segment_offsets.clone(); //Vec::new();
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
