use std::string::String as String;
use std::str;

use common::identifier;
use core::packet::typespace;

// VDD types: KeyID, PublicKey, Certificate, KeyName, SigTime

pub struct Validation {
    type: typespace::ValidationType,
    key_blob: Vec<u8>,
    cert_blob: Vec<u8>,
    key_id: identifier::Identifier,
    sig_time: u64,
    // TODO(cawood): implement the key link
    // key_link: u64,
}

impl Validation {
    pub fn decode(slice: &[u8], mut offset: usize) -> Result<Validation, DecoderError> {
        let mut vdd = Validation {
            type: typespace::ValidationType,
            key_blob: Vec::new(),
            cert_blob: Vec::new(),
            key_id: identifier::empty(),
            sig_time: 0,
        };

        let mut val_type: u16 = decoder::decode_tlv_parse_two(slice, offset); offset += 2;
        let mut val_length: u16 = decoder::decode_tlv_parse_two(slice, offset); offset += 2;
        vdd.type = typespace::ParseValidationType(vdd_type);

        while offset < val_length as usize {
            offset = decode_vdd_type(vdd, slice, offset)
            if offset == 0 {
                // XXX: error
            }
        }
    }

    fn decode_vdd_type(vdd: &mut Validation, slice &[u8], mut offset: usize) -> usize {
        let mut vdd_type: u16 = decoder::decode_tlv_parse_two(slice, offset); offset += 2;
        let mut vdd_length: u16 = decoder::decode_tlv_parse_two(slice, offset); offset += 2;
        let vdd_enum = typespace::ParseValidationDependentDataType(vdd_type)

        match vdd_emum {
            message::ValidationDependentDataType::KeyId => decode_vdd_key_id(vdd, slice, offset, vdd_length),
            message::ValidationDependentDataType::PublicKey => decode_vdd_key_public_key(vdd, slice, offset, vdd_length),
            message::ValidationDependentDataType::Certificate => decode_vdd_certificate(vdd, slice, offset, vdd_length),
            message::ValidationDependentDataType::KeyName => decode_vdd_key_name(vdd, slice, offset, vdd_length),
            message::ValidationDependentDataType::SignatureTime => decode_vdd_signature(vdd, slice, offset, vdd_length)
            message::ValidationDependentDataType::Invalid => return 0
        }
    }

    fn decode_vdd_key_id(vdd: &mut Validation, slice &[u8], mut offset: usize, length: usize) -> usize {
        return 0
    }

    fn decode_vdd_key_public_key(vdd: &mut Validation, slice &[u8], mut offset: usize, length: usize) -> usize {
        vdd.key_blob = 
        return 0
    }

    fn decode_vdd_certificate(vdd: &mut Validation, slice &[u8], mut offset: usize, length: usize) -> usize {
        return 0
    }

    fn decode_vdd_key_name(vdd: &mut Validation, slice &[u8], mut offset: usize, length: usize) -> usize {
        return 0
    }

    fn decode_vdd_signature(vdd: &mut Validation, slice &[u8], mut offset: usize, length: usize) -> usize {
        return 0
    }
}
