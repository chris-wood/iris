mod common;
mod control;
mod core;

use core::datastructure::fib as fib;
use core::datastructure::pit as pit;
use core::datastructure::cs as cs;

use std::env;
use std::thread;
use std::thread::JoinHandle;
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::sync::mpsc::{Sender,Receiver};
use std::sync::mpsc;

use std::net::{SocketAddrV4, UdpSocket, IpAddr, Ipv4Addr};

fn setup_listeners() -> Vec<JoinHandle<()>> {
    let mut listeners = Vec::new();

    // Create listeners for all faces
    let localhost = Ipv4Addr::new(127,0,0,1);
    let default_udp_addr = std::net::SocketAddrV4::new(localhost, 9696);
    let default_udp_listener = core::link::UDPLinkListener::new(default_udp_addr);
    let listen_result = default_udp_listener.listen();
    match listen_result {
        Ok(listener) => {
            println!("Created the UDP listener");
            listeners.push(listener);
        },
        Err(e) => {
            println!("Could not listen on UDP path");
        }
    }

    let default_tcp_addr = std::net::SocketAddrV4::new(localhost, 9697);
    let default_tcp_listener = core::link::TCPLinkListener::new(default_tcp_addr);
    let listen_result = default_tcp_listener.listen();
    match listen_result {
        Ok(listener) => {
            println!("Created the TCP listener");
            listeners.push(listener);
        },
        Err(e) => {
            println!("Could not listen on TCP path");
        }
    }

    return listeners;
}

fn setup_control() {
    control::control_repl();
}

fn run(listeners: Vec<JoinHandle<()>>) {
    println!("Joining on the listeners");
    for listener in listeners {
        listener.join();
    }
}

fn main() {
    println!("iris v0.0.1");

    let fcs = cs::Cache::new(0);
    let fpit = pit::PIT::new();
    let ffib = fib::FIB::new();

    // Create the forwarder and message processor
    let mut fwd = core::Forwarder::new(&fcs, &fpit, &ffib);

    let (tx, rx): (Sender<core::packet::message::Message>, Receiver<core::packet::message::Message>) = mpsc::channel();

    let processor = core::processor::Processor::new(&mut fwd, rx);

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
    processor.process_message(msg);

    //let listeners = setup_listeners();
    //run(listeners);

//    let name = msg.get_name();
//    let mut index = 0;
//    while index < name.len() {
//        println!("{}", name.at(index));
//        index = index + 1;
//    }
}
