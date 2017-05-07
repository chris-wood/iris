mod decoder;
mod validation;
mod bef;
pub mod message;
pub mod typespace;

use common::identifier;
use common::name;

pub fn decode(slice: &[u8]) -> Result<Packet, decoder::DecoderError> {
    return Packet::decode(slice, 0);
}

#[derive(PartialEq, Clone, Debug)]
struct FixedHeader {
    packet_type: typespace::PacketType,
    packet_specific_bytes: [u8; 3],
}

impl FixedHeader {
    fn empty() -> FixedHeader {
        FixedHeader {
            packet_type: typespace::PacketType::Interest,
            packet_specific_bytes: [0,0,0],
        }
    }
}

#[derive(Clone, Debug)]
pub struct Packet {
    bytes: Vec<u8>,
    header: FixedHeader,
    message: message::Message,
    validation: validation::Validation,
    bef: bef::BeginEndFragment,

    pub identifier: identifier::Identifier,

    // XXX: create slots for the message hash and sensitive region hash
}

impl Packet {
    pub fn name(&self) -> name::Name {
        return self.identifier.name.clone();
    }

    pub fn size(&self) -> usize {
        return self.bytes.len();
    }

    fn wrap(bytes: &[u8]) -> Packet {
        Packet {
            bytes: bytes.to_vec(),
            header: FixedHeader::empty(),
            message: message::Message::empty(),
            validation: validation::Validation::empty(),
            bef: bef::BeginEndFragment::empty(),
            identifier: identifier::Identifier::empty(),
        }
    }

    pub fn decode(slice: &[u8], mut offset: usize) -> Result<Packet, decoder::DecoderError> {
        // Simple bounds check
        if slice.len() < 8 {
            return Err(decoder::DecoderError::MalformedPacket);
        }

        // Decode the version and packet type
        let version: u8 = decoder::read_u8(slice, offset); offset += 1;
        let packet_type: u8 = decoder::read_u8(slice, offset); offset += 1;

        // Only proceed if we have a valid packet type
        match typespace::ParsePacketType(packet_type) {
            typespace::PacketType::Invalid => Err(decoder::DecoderError::MalformedPacket),
            packet_type => {
                // Parse out the header
                let mut plength: u16 = decoder::read_u16(slice, offset); offset += 2;
                plength = plength - 8; // packet length includes the fixed header

                // Parse the packet-specific fields
                let b1: u8 = decoder::read_u8(slice, offset); offset += 1;
                let b2: u8 = decoder::read_u8(slice, offset); offset += 1;
                let b3: u8 = decoder::read_u8(slice, offset); offset += 1;

                let header_length: u8 = decoder::read_u8(slice, offset); offset += 1;

                // Sanity check on the length
                if slice.len() != (offset + plength as usize) {
                    return Err(decoder::DecoderError::MalformedPacket)
                }

                // Create the empty packet
                let mut packet = Packet::wrap(slice);
                packet.header.packet_type = packet_type;
                packet.header.packet_specific_bytes[0] = b1;
                packet.header.packet_specific_bytes[1] = b2;
                packet.header.packet_specific_bytes[2] = b3;

                // Loop while haven't decoded the entire thing
                let start_offset = offset;
                while offset < plength as usize {
                    match packet.decode_tlv_toplevel(slice, plength, offset) {
                        Ok(n) => {
                            offset = n
                        },
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }

                // If we've run over the packet length, fail out
                if offset > (start_offset + plength as usize) {
                    return Err(decoder::DecoderError::MalformedPacket);
                }

                return Ok(packet);
            }
        }
    }

    fn decode_tlv_toplevel(&mut self, slice: &[u8], plength: u16, mut offset: usize) -> Result<usize, decoder::DecoderError> {
        while offset < (plength as usize) {
            let top_type: u16 = decoder::read_u16(slice, offset); offset += 2;
            let top_length: u16 = decoder::read_u16(slice, offset); offset += 2;

            if top_type == (typespace::TopLevelType::Interest as u16) {
                match message::Message::decode(slice, top_length, offset) {
                    Ok(msg) => { self.message = msg },
                    Err(e) => return Err(e)
                }
            } else if top_type == (typespace::TopLevelType::ContentObject as u16) {
                match message::Message::decode(slice, top_length, offset) {
                    Ok(msg) => { self.message = msg },
                    Err(e) => return Err(e)
                }
            } else if top_type == (typespace::TopLevelType::ValidationAlgorithm as u16) {
                match validation::Validation::decode(slice, top_length, offset) {
                    Ok(validation) => self.validation = validation,
                    Err(e) => return Err(e)
                }
            } else if top_type == (typespace::TopLevelType::BeginEndFragment as u16) {
                match bef::BeginEndFragment::decode(slice, top_length, offset) {
                    Ok(bef) => self.bef = bef,
                    Err(e) => return Err(e)
                }
            } else if top_type == (typespace::TopLevelType::ValidationPayload as u16) {
                // TODO(caw): decode the validation payload and store it
            } else {
                // Swallow this unknown TLV and continue onwards
            }

            offset += top_length as usize;
        }

        return Ok(offset)
    }

    pub fn get_packet_type(&self) -> typespace::PacketType {
        self.header.packet_type.clone()
    }
}
