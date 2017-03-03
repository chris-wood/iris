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
pub enum ValidationType {
    Crc32 = 0x0002,
    HmacSha256 = 0x0004,
    Vmac128 = 0x0005,
    RsaSha256 = 0x0006,
    EcSecp256K1 = 0x0007,
    EcSecp384R1 = 0x0008,
    Invalid = 0xFFFF
}

pub fn ParseValidationType(val: u16) -> ValidationType {
    match val {
        0x0002 => ValidationType::Crc32,
        0x0004 => ValidationType::HmacSha256,
        0x0005 => ValidationType::Vmac128,
        0x0006 => ValidationType::RsaSha256,
        0x0007 => ValidationType::EcSecp256K1,
        0x0008 => ValidationType::EcSecp384R1,
        _      => ValidationType::Invalid
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum ValidationDependentDataType {
    KeyId = 0x0009,
    PublicKey = 0x000B,
    Certificate = 0x000C,
    KeyName = 0x000E,
    SignatureTime = 0x000F,
    Invalid = 0xFFFF
}

pub fn ParseValidationDependentDataType(val: u16) -> ValidationDependentDataType {
    match val {
        0x0009 => ValidationDependentDataType::KeyId,
        0x000B => ValidationDependentDataType::PublicKey,
        0x000C => ValidationDependentDataType::Certificate,
        0x000E => ValidationDependentDataType::KeyName,
        0x000F => ValidationDependentDataType::SignatureTime,
        _      => ValidationDependentDataType::Invalid,
    }
}
