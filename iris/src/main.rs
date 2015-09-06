mod cs;
mod fib;
mod fwd;
pub mod packet;
mod pit;

use std::env;
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

fn main() {
    println!("IRIS v0.0.1");

    let args: Vec<String> = env::args().collect();
    // let file_name: String = String::from(args[1]);

    let path = Path::new(&args[1]);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    let mut file_contents = Vec::new();
    match file.read_to_end(&mut file_contents) {
        Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
        Ok(val) => print!("{}", val)
    }

    let buffer = &file_contents[..];

    // simple TLV
    //let xs: [u8; 5] = [1, 2, 0, 1, 0]; // (1 << 8) | 2
    //packet::decode_packet(&xs);

    packet::decode_packet(buffer);
}
