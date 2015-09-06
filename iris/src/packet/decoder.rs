

// TODO: this should just be decode_tlv
pub fn decode_packet_intro(slice: &[u8], offset: usize) {
    let ptype: u16 = ((slice[offset] as u16) << 8) | (slice[offset + 1] as u16);
    let plength: u16 = ((slice[offset + 2] as u16) << 8) | (slice[offset + 3] as u16);

    if (ptype >> 8) as u8 == 1 {
        print!("content object\n");
    } else if (ptype >> 8) as u8 == 1 {
        print!("interest\n");
    }

    decode_tlv_header(ptype, plength, &slice[(offset + 4) .. (offset + 4 + plength as usize) as usize]);
}

// fn decode_tlv(ptype: u16, plength: u16, pvalue: &[u8]) {
//     paass
// }

fn decode_tlv_header(ptype: u16, plength: u16, pvalue: &[u8]) {
    println!("TLV = {} {} {}", ptype, plength, pvalue[0]);
    // 257 = 0000000100000001 (version=1, ptype=1)
}
