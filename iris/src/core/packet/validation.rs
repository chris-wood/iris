use std::string::String as String;
use std::str;

use common::identifier;
use core::packet::typespace;
use core::packet::decoder;

#[derive(Clone, Debug)]
pub struct Validation {
    validation_type: typespace::ValidationType,
    key_blob: Vec<u8>,
    cert_blob: Vec<u8>,
    key_id: identifier::Identifier,
    sig_time: u64,
    // TODO(cawood): implement the key link
}

impl Validation {
    pub fn empty() -> Validation {
        Validation {
            validation_type: typespace::ValidationType::Invalid,
            key_blob: Vec::new(),
            cert_blob: Vec::new(),
            key_id: identifier::Identifier::empty(),
            sig_time: 0,
        }
    }

    pub fn decode(slice: &[u8], length: u16, mut offset: usize) -> Result<Validation, decoder::DecoderError> {
        let mut vdd = Validation {
            validation_type: typespace::ValidationType::Invalid,
            key_blob: Vec::new(),
            cert_blob: Vec::new(),
            key_id: identifier::Identifier::empty(),
            sig_time: 0,
        };

        let mut val_type: u16 = decoder::read_u16(slice, offset); offset += 2;
        let mut val_length: u16 = decoder::read_u16(slice, offset); offset += 2;
        vdd.validation_type = typespace::ParseValidationType(val_length);

        while offset < val_length as usize && offset < length as usize {
            offset = vdd.decode_vdd_type(slice, offset);
            if offset == 0 {
                return Err(decoder::DecoderError::MalformedPacket);
            }
        }

        if offset > length as usize {
            return Err(decoder::DecoderError::MalformedPacket);
        }

        return Ok(vdd);
    }

    fn decode_vdd_type(&mut self, slice: &[u8], mut offset: usize) -> usize {
        let mut vdd_type: u16 = decoder::read_u16(slice, offset); offset += 2;
        let mut vdd_length: u16 = decoder::read_u16(slice, offset); offset += 2;

        match typespace::ParseValidationDependentDataType(vdd_type) {
            typespace::ValidationDependentDataType::KeyId => self.decode_vdd_key_id(slice, offset, vdd_length),
            typespace::ValidationDependentDataType::PublicKey => self.decode_vdd_key_public_key(slice, offset, vdd_length),
            typespace::ValidationDependentDataType::Certificate => self.decode_vdd_certificate(slice, offset, vdd_length),
            typespace::ValidationDependentDataType::KeyName => self.decode_vdd_key_name(slice, offset, vdd_length),
            typespace::ValidationDependentDataType::SignatureTime => self.decode_vdd_signature(slice, offset, vdd_length),
            typespace::ValidationDependentDataType::Invalid => return 0
        }
    }

    fn decode_vdd_key_id(&mut self, slice: &[u8], mut offset: usize, length: u16) -> usize {
        return 0
    }

    fn decode_vdd_key_public_key(&mut self, slice: &[u8], mut offset: usize, length: u16) -> usize {
        // XXX: TODO: decode the public key bytes and put them in the proper place
        return 0
    }

    fn decode_vdd_certificate(&mut self, slice: &[u8], mut offset: usize, length: u16) -> usize {
        return 0
    }

    fn decode_vdd_key_name(&mut self, slice: &[u8], mut offset: usize, length: u16) -> usize {
        return 0
    }

    fn decode_vdd_signature(&mut self, slice: &[u8], mut offset: usize, length: u16) -> usize {
        return 0
    }
}
