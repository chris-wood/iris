mod decoder;
mod validation;
pub mod message;
pub mod typespace;

use common::identifier;
use common::name;

pub fn decode(slice: &[u8]) -> Result<Packet, decoder::DecoderError> {
    return Packet::decode(slice, 0);
}

#[derive(Clone, Debug)]
pub struct Packet {
    bytes: Vec<u8>,
    pub packet_type: typespace::PacketType,
    message: message::Message,
    validation: validation::Validation,
    pub identifier: identifier::Identifier,
    // XXX: create slots for the message hash and sensitive region hash
}

impl Packet {

    // XXX: restrict the lifetime to prevent clones
    pub fn name(&self) -> name::Name {
        return self.identifier.name.clone();
    }

    pub fn size(&self) -> usize {
        return self.bytes.len();
    }

    fn wrap(bytes: &[u8]) -> Packet {
        Packet {
            bytes: bytes.to_vec(),
            packet_type: typespace::PacketType::Interest,
            message: message::Message::empty(),
            validation: validation::Validation::empty(),
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
                let rsvd: u16 = decoder::read_u16(slice, offset); offset += 2;
                let flags: u8 = decoder::read_u8(slice, offset); offset += 1;
                let header_length: u8 = decoder::read_u8(slice, offset); offset += 1;

                // Sanity check on the length
                if slice.len() != (offset + plength as usize) {
                    return Err(decoder::DecoderError::MalformedPacket)
                }

                // Create the empty packet
                let mut packet = Packet::wrap(slice);
                packet.packet_type = packet_type;

                // Loop while haven't decoded the entire thing
                while offset < plength as usize {
                    match packet.decode_tlv_toplevel(slice, plength, offset) {
                        Ok(n) => offset = offset + n,
                        Err(e) => return Err(e),
                    }
                }

                // If we've run over the packet length, fail out
                if offset > plength as usize {
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
            offset += top_length as usize;

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
            } else if top_type == (typespace::TopLevelType::ValidationPayload as u16) {
                // TODO(caw): decode the validation payload and store it
            } else {
                // Swallow this unknown TLV and continue onwards
            }
        }

        return Ok(offset)
    }
}
