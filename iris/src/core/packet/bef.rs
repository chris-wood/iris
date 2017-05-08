use std::string::String as String;
use std::str;

use common::identifier;
use core::packet::typespace;
use core::packet::decoder;

#[derive(Clone, Debug)]
pub struct BeginEndFragment {
    content_id: Vec<u8>, // 32B content ID
    offset: u16, // data offset in the overall stream
    fragment_bytes: Vec<u8>, // the actual bytes to store
}

impl BeginEndFragment {
    pub fn empty() -> BeginEndFragment {
        BeginEndFragment {
            content_id: Vec::new(),
            offset: 0,
            fragment_bytes: Vec::new(),
        }
    }

    pub fn decode(slice: &[u8], length: u16, mut offset: usize) -> Result<BeginEndFragment, decoder::DecoderError> {
        // Read the content ID
        let content_id = decoder::read(slice, offset, 32); offset += 32;
        // XXX: check for remaining byets
        // return Err(decoder::DecoderError::MalformedPacket);

        // Read the offfet
        let frag_offset = decoder::read_u16(slice, offset); offset += 2;
        // XXX: check for remaining byets

        // Read the remaints of the data, up to the length
        let contents = decoder::read(slice, offset, (length as usize) - offset); offset = length as usize;

        let mut bef = BeginEndFragment {
            content_id: content_id.to_vec(),
            offset: frag_offset,
            fragment_bytes: contents.to_vec(),
        };

        return Ok(bef);
    }
}
