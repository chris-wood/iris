use std::vec;
use std::thread;
use std::thread::JoinHandle;
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

pub trait Link {
    // TODO: maybe rename run() to receive_from()?
    fn run(&mut self);
    fn stop(&mut self);
    fn send_to(&mut self, wire_format: &[u8]) -> usize;
}

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

pub struct TCPLink {
    linkId: u16,
    kill: bool,
    stream: TcpStream,
    dst: SocketAddr,
    channel: Sender<(core::packet::message::Message, u16)>
}

impl TCPLink {
    pub fn new(id: u16, stream: TcpStream, dst: SocketAddr, sender: Sender<(core::packet::message::Message, u16)>) -> (Box<Link>) {
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

pub enum LinkListenerError {
    CouldNotSpawnThread,
    CouldNotCloneSocket
}

pub trait LinkListener {
    fn listen(&self) -> Result<JoinHandle<()>, LinkListenerError>;
}

pub struct UDPLinkListener {
    address: SocketAddrV4,
    socket: UdpSocket,
    channel: Sender<(core::packet::message::Message, u16)>
}

impl UDPLinkListener {
    pub fn new(sender: Sender<(core::packet::message::Message, u16)>, addr: SocketAddrV4) -> Box<LinkListener> {
        let attempt = UdpSocket::bind(addr);
        let mut socket;
        match attempt {
            Ok(sock) => {
                println!("Created the socket");
                socket = sock;
            },
            Err(err) => {
                panic!(format!("Unable to bind to source"));
            }
        }

        let listener = UDPLinkListener {
            address: addr,
            socket: socket,
            channel: sender
        };
        return Box::new(listener);
    }
}

impl LinkListener for UDPLinkListener {
    fn listen(&self) -> Result<JoinHandle<()>, LinkListenerError> {
        let clone = self.socket.try_clone();
        match clone {
            Ok(socket) => {
                let channel = self.channel.clone();
                let listenerThread = thread::spawn(move || {
                    println!("Inner UDP loop listener");
                    let mut buf = [0; 4096]; // 4k MTU for UDP, by default
                    loop {
                        match socket.recv_from(&mut buf) {
                            Ok((amt, src)) => {
                                println!("Got a new connection message");
                                let clone = socket.try_clone();
                                let channel_clone = channel.clone();
                                match clone {
                                    Ok(socket) => {
                                        println!("Cloned OK--starting a link.");
                                        thread::spawn(move || {
                                            print!("start the new UDP link!");
                                            let mut link = UDPLink::new(0, socket, src, channel_clone);
                                            link.run();
                                        });
                                    },
                                    Err(e) => {
                                        println!("Couldn't clone the UdpSocket to start the link");
                                    }
                                }
                            },
                            Err(e) => {
                                println!("couldn't recieve a datagram: {}", e);
                            }
                        }
                    };
                    println!("done with the loop?...");
                });
                return Ok(listenerThread);
            },
            Err(e) => {
                return Err(LinkListenerError::CouldNotCloneSocket);
            }
        }
        return Err(LinkListenerError::CouldNotSpawnThread);
    }
}

pub struct TCPLinkListener {
    listener: TcpListener,
    address: SocketAddrV4,
    channel: Sender<(core::packet::message::Message, u16)>
}

impl TCPLinkListener {
    pub fn new(sender: Sender<(core::packet::message::Message, u16)>, addr: SocketAddrV4) -> Box<LinkListener> {
        let attempt = TcpListener::bind(addr);
        let mut streamListener;
        match attempt {
            Ok(newListener) => {
                println!("Created the TCP stream listener");
                streamListener = newListener;
            },
            Err(err) => {
                panic!(format!("Unable to bind to source"));
            }
        }

        let listener = TCPLinkListener {
            listener: streamListener,
            address: addr,
            channel: sender
        };
        return Box::new(listener);
    }
}

impl LinkListener for TCPLinkListener {
    fn listen(&self) -> Result<JoinHandle<()>, LinkListenerError> {
        let listener = self.listener.try_clone();
        let channel = self.channel.clone();
        match listener {
            Ok(realListener) => {
                let listenerThread = thread::spawn(move || {
                    for stream in realListener.incoming() {
                        match stream {
                            Ok(stream) => {
                                let addr = stream.peer_addr().unwrap();
                                let channel_clone = channel.clone();
                                thread::spawn(move || {
                                    print!("start the link!");
                                    let mut link = TCPLink::new(0, stream, addr, channel_clone);
                                    link.run();
                                });
                            }
                            Err(e) => {
                                panic!(format!("Failed to create a new stream from the listener"));
                            }
                        }
                    }
                });
                return Ok(listenerThread);
            },
            Err(e) => {
                return Err(LinkListenerError::CouldNotCloneSocket);
            }
        }
    }
}

// pub struct IPLinkListener {
//
// }
//
// pub struct EthLinkListener {
//
// }
