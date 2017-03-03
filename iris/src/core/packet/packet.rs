use core::packet::decoder as decoder;
use core::packet::message as message;
use core::packet::validation as validation;
use core::packet::typespace as typespace;
use common::identifier;

#[derive(Clone, Debug)]
pub struct Packet {
    bytes: Vec::new()
    ptype: typespace::PacketType,
    message: message::Message,
    validation: validation::Validation,
    identifier: identifier::Identifier,
}

impl Packet {
    fn new(ptype: typespace::PacketType, msg: message::Message, val: validation::Validation) -> Packet {
        // XXX: construct the identifier from the message and validation information
        // XXX: need to pass the raw bytes into this function so they may be stored

        Packet {
            ptype: ptype,
            message: msg,
            validation: val,
            idenfifier: ident,
        }
    }

    pub fn decode(slice: &[u8], mut offset: usize) -> Result<Packet, DecoderError> {
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
                while offset < plength {
                    // XXX: invoke the decode function
                    let consumed: usize = decode_tlv_toplevel(&mut msg, slice, plength, offset);
                }

                // If we've run over the packet length, fail out
                if offset > plength {
                    Err(decoder::DecoderError::MalformedPacket)
                }

                // XXX: return the message
                Ok(Packet::new(ptype, msg, val, ident))
            }
        }
    }
}
