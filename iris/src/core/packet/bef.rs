use std::string::String as String;
use std::str;

use common::identifier;
use core::packet::typespace;
use core::packet::decoder;

#[derive(Clone, Debug)]
pub struct BeginEndFragment {
    // TODO(cawood): implement the key link
}

impl BeginEndFragment {
    pub fn empty() -> BeginEndFragment {
        BeginEndFragment {

        }
    }

    pub fn decode(slice: &[u8], length: u16, mut offset: usize) -> Result<BeginEndFragment, decoder::DecoderError> {
        let mut bef = BeginEndFragment {

        };

        // let mut val_type: u16 = decoder::read_u16(slice, offset); offset += 2;
        // let mut val_length: u16 = decoder::read_u16(slice, offset); offset += 2;
        // vdd.validation_type = typespace::ParseValidationType(val_length);
        //
        // while offset < val_length as usize && offset < length as usize {
        //     offset = vdd.decode_vdd_type(slice, offset);
        //     if offset == 0 {
        //         return Err(decoder::DecoderError::MalformedPacket);
        //     }
        // }
        //
        // if offset > length as usize {
        //     return Err(decoder::DecoderError::MalformedPacket);
        // }

        return Ok(bef);
    }
}
