pub enum TopLevelType {
    Interest = 0x0001,
    ContentObject = 0x0002,
    ValidationAlgorithm = 0x0003,
    ValidationPayload = 0x0004
}

pub enum PacketType {
    Interest = 0x0000,
    ContentObject = 0x0001,
    InterestReturn = 0x0002
}

enum HopByHopHeaderType {
    InterestLifetime = 0x0001,
    CacheTime = 0x0002
}

enum MessageType {
    Name = 0x0000,
    Payload = 0x0001
}

enum NameType {
    NameSegment = 0x0001,
    PayloadID = 0x0002,
    AppLower = 0x1000,
    AppUpper = 0x1FFF
}

enum InterestMessageTLVType {
    KeyIdRestriction = 0x0002,
    HashRestriction = 0x0003
}

enum ContentObjectMessageTLVType {
    PayloadType = 0x0005,
    ExpiryTime = 0x0006
}

enum ValidationType {
    CRC32 = 0x0002,
    HMAC_SHA256 = 0x0004,
    VMAC_128 = 0x0005,
    RSA_SHA256 = 0x0006,
    EC_SECP_256K1 = 0x0007,
    EC_SECP_384R1 = 0x0008,
}

enum ValidationDependentDataType {
    KeyId = 0x0009,
    PublicKey = 0x000B,
    Certificate = 0x000C,
    KeyName = 0x000E,
    SignatureTime = 0x000F
}

pub struct Message {
    pub packet_type: PacketType,
    pub name_offset: usize,
    pub name_length: usize,
    pub payload_offset: usize,
    pub payload_length: usize,
    pub validation_offset: usize,
    pub validation_length: usize
}

impl Message {
    // A public constructor
    pub fn new(new_packet_type: PacketType, new_payload_offset: usize, new_payload_length: usize, new_validation_offset: usize, new_validation_length: usize) -> Message {
        Message {
            name_offset: 0,
            name_length: 0,
            packet_type: new_packet_type,
            payload_offset: new_payload_offset,
            payload_length: new_payload_length,
            validation_offset: new_validation_offset,
            validation_length: new_validation_length
        }
    }

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
