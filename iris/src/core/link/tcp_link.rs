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


pub struct TCPLink {
    linkId: u16,
    kill: bool,
    stream: TcpStream,
    dst: Option<SocketAddr>,
    channel: Sender<(core::packet::message::Message, u16)>
}

impl TCPLink {
    pub fn new(id: u16, stream: TcpStream, dst: Option<SocketAddr>, sender: Sender<(core::packet::message::Message, u16)>) -> (Box<Link>) {
        let link = TCPLink {
            linkId: id,
            kill: false,
            stream: stream,
            dst: dst,
            channel: sender
        };
        return Box::new(link);
    }
}


impl Link for TCPLink {
    fn send_to(&mut self, wire_format: &[u8]) -> (usize) {
        let result = self.stream.write(wire_format);
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
        let mut buf = [0; 8192]; // 8k MTU for testing
        loop {
            let result = self.stream.read(&mut buf);
            match result {
                Ok(num_bytes) => {
                    if num_bytes > 0 {
                        println!("read {} bytes -- converting to a packet (or dropping)", num_bytes);
                        let msg = core::packet::decode_packet(&buf[0..num_bytes]);
                        let result = self.channel.send((msg, self.linkId));
                        match result {
                            Ok(m) => {
                                println!("Sent message to the processor.");
                            },
                            Err(e) => {
                                println!("Error: unable to send message to the processor.");
                            }
                        }
                    } else {
                        // println!("Read {} bytes!", num_bytes);
                    }
                },
                Err(e) => {
                    println!("read Error! {}", e);
                }
            }
        }
    }
}
