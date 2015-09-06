pub fn decode_packet(slice: &[u8], offset: usize) { // TODO: this should just be decode_tlv
    // TODO: move these into functions
    let ptype: u16 = ((slice[offset] as u16) << 8) | (slice[offset + 1] as u16);
    let plength: u16 = ((slice[offset + 2] as u16) << 8) | (slice[offset + 3] as u16);
    decode_tlv(ptype, plength, &slice[(offset + 4) .. (offset + 4 + plength as usize) as usize])
}

fn decode_tlv(ptype: u16, plength: u16, pvalue: &[u8]) {
    println!("here, {} {} {}", ptype, plength, pvalue[0]);
}
