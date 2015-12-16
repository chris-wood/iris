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

fn setup_listeners(sender: Sender<(core::packet::message::Message, u16)>) -> Vec<JoinHandle<()>> {
    let mut listeners = Vec::new();

    // Create listeners for all faces
    let localhost = Ipv4Addr::new(127,0,0,1);
    let default_udp_addr = std::net::SocketAddrV4::new(localhost, 9696);
    let default_udp_listener = core::link::UDPLinkListener::new(sender.clone(), default_udp_addr);
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
    let default_tcp_listener = core::link::TCPLinkListener::new(sender.clone(), default_tcp_addr);
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
    // Create the default data structures
    let mut fcs = cs::Cache::new(0);
    let mut fpit = pit::PIT::new();
    let mut ffib = fib::FIB::new();

    // Create the forwarder and message processor
    // Note; the forwarder now owns all three structures.
    let mut fwd = core::Forwarder::new(&fcs, &mut fpit, ffib);

    // Create the send/receive channel
    let (tx, rx): (Sender<(core::packet::message::Message, u16)>, Receiver<(core::packet::message::Message, u16)>) = mpsc::channel();

    // Create the processor to handle the link <-> core message passing
    // Move the receiver into the processor
    let mut processor = core::processor::Processor::new(fwd, rx);

    // let args: Vec<String> = env::args().collect();
    //
    // let path = Path::new(&args[1]);
    // let display = path.display();
    //
    // let mut file = match File::open(&path) {
    //     Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
    //     Ok(file) => file,
    // };
    //
    // let mut file_contents = Vec::new();
    // match file.read_to_end(&mut file_contents) {
    //     Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
    //     Ok(_) => {}
    // }
    //
    // let buffer = &file_contents[..]; // take reference to the entire thing (i.e., a slice)
    //
    // let msg = core::packet::decode_packet(buffer);
    // processor.process_message(msg, 1);

    // thread::spawn(move || {
    //
    // });
    thread::spawn(move || {
        setup_control();
    });

    // TODO: maybe run should accept these listeners? Listeners: on new connection, create link and tell processor about the new ID and Link
    let listeners = setup_listeners(tx);
    processor.run();
    run(listeners);
}
