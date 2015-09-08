enum TopLevelType {
    Interest = 0x0001,
    ContentObject = 0x0002
    ValidationAlgorithm = 0x0003,
    ValidationPayload = 0x0004
}

enum PacketType {
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

struct Message {
    packet_type: PacketType,
    // TODO: what else?
}
