use core::packet::decoder;
use core::packet::message;
use core::packet::validation;
use core::packet::typespace;
use common::identifier;

#[derive(Clone, Debug)]
pub struct Packet {
    bytes: Vec<u8>,
    ptype: typespace::PacketType,
    message: message::Message,
    validation: validation::Validation,
    identifier: identifier::Identifier,
}

impl Packet {
    fn new(bytes: &[u8], ptype: typespace::PacketType, msg: message::Message, val: validation::Validation) -> Packet {
        // XXX: construct the identifier from the message and validation information
        // XXX: need to pass the raw bytes into this function so they may be stored!

        Packet {
            bytes: bytes.to_vec(),
            ptype: ptype,
            message: msg,
            validation: val,
            identifier: ident,
        }
    }

    pub fn decode(slice: &[u8], mut offset: usize) -> Result<Packet, decoder::DecoderError> {
        // Simple bounds check
        if slice.len() < 8 {
            Err(decoder::DecoderError::MalformedPacket)
        }

        // Decode the version and packet type
        let version: u8 = decoder::read_u8(slice, offset); offset += 1;
        let ptype: u8 = decoder::read_u8(slice, offset); offset += 1;

        // Only proceed if we have a valid packet type
        match ParsePacketType(packet_type) {
            typespace::PacketType::Invalid => Err(decoder::DecoderError::MalformedPacket),
            ptype => {
                // Parse out the header
                let mut plength: u16 = decoder::read_u16(slice, offset); offset += 2;
                plength = plength - 8; // packet length includes the fixed header
                let rsvd: u16 = decoder::read_u16(slice, offset); offset += 2;
                let flags: u8 = decoder::read_u8(slice, offset); offset += 1;
                let header_length: u8 = decoder::read_u8(slice, offset); offset += 1;

                // Sanity check on the length
                if slice.len() != (offset + plength as usize) {
                    return Err(DecoderError::MalformedPacket)
                }

                // Loop while haven't decoded the entire thing
                while offset < plength as usize {
                    // XXX: invoke the decode function
                    let consumed: usize = decode_tlv_toplevel(&mut msg, slice, plength, offset);
                }

                // If we've run over the packet length, fail out
                if offset > plength as usize {
                    Err(decoder::DecoderError::MalformedPacket)
                }

                // XXX: return the message
                Ok(Packet::new(ptype, msg, val, ident))
            }
        }
    }

    fn decode_tlv_toplevel(&mut self, slice: &[u8], plength: u16, mut offset: usize) -> (usize) {
        while offset < (plength as usize) {
            let top_type: u16 = decoder::read_u16(slice, offset); offset += 2;
            let top_length: u16 = decoder::read_u16(slice, offset); offset += 2;

            if top_type == (typespace::TopLevelType::Interest as u16) {
                match message::Message::decode(slice, top_length, offset) {
                    Ok(msg) => self.message = msg,
                    Err(_) => return 0
                }
            } else if top_type == (typespace::TopLevelType::ContentObject as u16) {
                match message::Message::decode(slice, top_length, offset) {
                    Ok(msg) => self.message = msg,
                    Err(_) => return 0
                }
            } else if top_type == (typespace::TopLevelType::ValidationAlgorithm as u16) {
                match validation::Validation::decode(slice, top_length, offset) {
                    Ok(validation) => self.validation = validation,
                    Err(_) => 0
                }
            } else if top_type == (typespace::TopLevelType::ValidationPayload as u16) {
                // offset = decode_tlv_validation_payload(self.validation_payload, slice, top_length, offset);
                // match validation::decode(slice, top_length, offset) {
                //     Ok(validation) => self.validation = validation,
                //     Err(_) => 0
                // }
            } else {
                // TODO: throw exception!
            }
        }

        return offset;
    }
}
