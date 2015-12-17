use core::link::Link as Link;

use std::io::Write;
use std::io::Read;

use std::net::UdpSocket;
use std::net::TcpStream;
use std::net::TcpListener;
use std::net::SocketAddr;
use std::net::SocketAddrV4;

use std::sync::mpsc::{Sender,Receiver};
use std::sync::mpsc;

use common::name;
use core;

pub struct UDPLink {
    linkId: u16,
    kill: bool,
    socket: UdpSocket,
    dst: SocketAddr,
    channel: Sender<(core::packet::message::Message, u16)>
}

impl UDPLink {
    pub fn new(id: u16, socket: UdpSocket, dst: SocketAddr, sender: Sender<(core::packet::message::Message, u16)>) -> (Box<Link>) {
        let link = UDPLink {
            linkId: id,
            kill: false,
            socket: socket,
            dst: dst,
            channel: sender
        };
        return Box::new(link);
    }
}


impl Link for UDPLink {
    fn send_to(&mut self, wire_format: &[u8]) -> (usize) {
        let result = self.socket.send_to(wire_format, self.dst);
        let mut numBytes;
        match result {
            Ok(bytes) => {
                println!("Sent {} bytes", bytes);
                numBytes = bytes;
            },
            Err(err) => {
                panic!("Failed to send to localhost:9999");
            }
        }
        return numBytes;
    }

    fn stop(&mut self) {
        self.kill = true;
    }

    fn run(&mut self) {
        println!("Inside the the UDP link run() function");
        let mut buf = [0; 4096]; // 4k MTU for UDP, by default
        loop {
            match self.socket.recv_from(&mut buf) {
                Ok((amt, src)) => {
                    println!("Got a message: {}", buf.len());
                },
                Err(e) => {
                    println!("Error: couldn't receive datagram {}", e);
                }
            }
        }
    }
}
