mod common;
mod control;
mod core;

use std::env;
use std::thread;
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::sync::mpsc::{Sender,Receiver};
use std::sync::mpsc;

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

    let msg = core::packet::decode_packet(buffer);
    msg.print();

    // Create the forwarder
    let fwd = core::Forwarder::new();
    let (tx, rx): (Sender<core::packet::message::Message>, Receiver<core::packet::message::Message>) = mpsc::channel();
    let processor = core::processor::Processor::new(fwd, rx);

    let mut listeners = Vec::new();

    // Create listeners for all faces
    let localhost = Ipv4Addr::new(127,0,0,1);
    let defaultUdpAddr = std::net::SocketAddrV4::new(localhost, 9696);
    let defaultUdpListener = core::link::UDPLinkListener::new(defaultUdpAddr);
    let listenResult = defaultUdpListener.listen();
    match listenResult {
        Ok(listener) => {
            println!("Created the UDP listener");
            listeners.push(listener);
        },
        Err(e) => {
            println!("Could not listen on UDP path");
        }
    }

    let defaultTcpAddr = std::net::SocketAddrV4::new(localhost, 9697);
    let defaultTcpListener = core::link::TCPLinkListener::new(defaultTcpAddr);
    let listenResult = defaultTcpListener.listen();
    match listenResult {
        Ok(listener) => {
            println!("Created the TCP listener");
            listeners.push(listener);
        },
        Err(e) => {
            println!("Could not listen on TCP path");
        }
    }

    // Open the command REPL
    control::control_repl();

    println!("Joining on the listeners");
    for listener in listeners {
        listener.join();
    }
}
