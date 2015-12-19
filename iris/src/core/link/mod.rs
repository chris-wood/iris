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

mod udp_link;
use core::link::udp_link::UDPLink as UDPLink;
mod tcp_link;
use core::link::tcp_link::TCPLink as TCPLink;

use std::collections::HashMap;

pub enum LinkType {
    UDP,
    TCP
}

pub struct LinkTable {
    links: HashMap<u16, Box<Link>> // http://doc.rust-lang.org/1.4.0/std/collections/struct.HashMap.html
}

impl LinkTable {
    pub fn new() -> LinkTable {
        LinkTable {
            links: HashMap::new()
        }
    }

    pub fn add_link(&mut self, id: u16, link: Box<Link>) {
        self.links.insert(id, link);
    }

    pub fn get_link(&mut self, id: u16) -> &Box<Link> {
        // if self.links.contains_key(id) {
        //
        // }
        return self.links.get(&id).unwrap();
    }

    pub fn get_new_id(&self) -> u16 {
        return self.links.len() as u16;
    }
}

pub trait Link {
    fn run(&mut self); // TODO: maybe rename run() to receive_from()?
    fn stop(&mut self);
    fn send_to(&mut self, wire_format: &[u8]) -> usize;
}

pub struct LinkManager {
    listeners: Vec<Box<LinkListener>>,
    table: LinkTable,
    channel: Sender<(core::packet::message::Message, u16)>,
    receiver: Receiver<(core::packet::message::Message, u16)>
}

impl LinkManager {
    pub fn new(sender: Sender<(core::packet::message::Message, u16)>, receiver: Receiver<(core::packet::message::Message, u16)>, mut table: LinkTable, listeners: Vec<Box<LinkListener>>) -> LinkManager {
        LinkManager {
            listeners: listeners,
            table: table,
            channel: sender,
            receiver: receiver
        }
    }

    pub fn add_link(&mut self, nick: String, link_type: LinkType, addr_string: String) -> bool {
        let id = self.table.get_new_id();

        // TODO: need to map the link to this ID, and need to map this id to the Link that's created...

        match link_type {
            LinkType::UDP => {
                // TODO: implement me!
                return false;
            }, LinkType::TCP => {
                let stream = TcpStream::connect(&addr_string[..]).unwrap();
                let channel_clone = self.channel.clone();

                // let (tx, rx): (Sender<core::packet::message::Message>, Receiver<core::packet::message::Message>) = mpsc::channel(); // for sending messages

                thread::spawn(move || {
                    print!("start the TCP link from the LinkManager!");
                    let mut link = TCPLink::new(id, stream, None, channel_clone);
                    link.run();
                });
            }
        }
        return false;
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
                                    let mut link = TCPLink::new(0, stream, Some(addr), channel_clone);
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
