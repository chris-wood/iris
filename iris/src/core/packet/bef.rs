use std::string::String as String;
use std::str;

use common::identifier;
use core::packet::typespace;
use core::packet::decoder;

#[derive(Clone, Debug)]
pub struct BeginEndFragment {
    // content_id: [u8; 32], // 32B content ID
    // offset: u16, // data offset in the overall stream
    // fragment_bytes: Vec<u8>, // the actual bytes to store
}

impl BeginEndFragment {
    pub fn empty() -> BeginEndFragment {
        BeginEndFragment {
            //
        }
    }

    pub fn decode(slice: &[u8], length: u16, mut offset: usize) -> Result<BeginEndFragment, decoder::DecoderError> {
        let mut bef = BeginEndFragment {
            // XXX
        };

// XXX: hash digest that points to the parent packet
// XXX: offset into the fragment
// XXX: data

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
