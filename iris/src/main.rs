mod packet;
mod common;
mod control;
mod core;

use std::env;
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

use std::net::{SocketAddrV4, UdpSocket, IpAddr, Ipv4Addr};

fn main() {
    println!("iris v0.0.1");

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
        Ok(_) => {}
    }

    let buffer = &file_contents[..]; // take reference to the entire thing (i.e., a slice)

    let msg = packet::decode_packet(buffer);
    msg.print();

    // Create the forwarder
    let fwd = core::Forwarder::new();

    // Create listeners for all faces
    let localhost = Ipv4Addr::new(127,0,0,1);
    let defaultUdpAddr = std::net::SocketAddrV4::new(localhost, 9696);
    let defaultUdpListener = core::link::UDPLinkListener::new(defaultUdpAddr);
    defaultUdpListener.listen();

    // Open the command REPL
    control::control_repl();
}
