extern crate byteorder;
use byteorder::{BigEndian, ReadBytesExt};

fn decode_packet(slice: &[u8]) {
    let byets = [slice[0], slice[1]];
    let ptype = buf.read_u16::<BigEndian>().unwrap();
    println!("value = {}", ptype);

    //println!("first element of the slice: {}", slice[0]);
    //println!("the slice has {} elements", slice.len());
}

fn main() {
    let xs: [u8; 2] = [1, 2];
    decode_packet(&xs[1..2]);
}
