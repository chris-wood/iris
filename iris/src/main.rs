extern crate mio;
extern crate bytes;

use mio::*; // http://nbaksalyar.github.io/2015/07/10/writing-chat-in-rust.html

use bytes::{Buf, RingBuf, SliceBuf, MutBuf};

use std::str;

mod common;
mod control;
mod core;

use core::datastructure::fib as fib;
use core::datastructure::pit as pit;
use core::datastructure::cs as cs;

use std::env;
use std::io::prelude::*;
use std::thread;
use std::thread::JoinHandle;
use std::error::Error;
use std::fs::File;
use std::path::Path;

use std::collections::HashMap;
use std::net::SocketAddr;
use mio::tcp::*;
use mio::udp::*;

enum ControlMessageError {
    InvalidControlMessage
}

enum LinkManagerError {
    CannnotAddLink
}

pub enum LinkType {
    UDP,
    TCP
}

struct TCPLink {
    socket: TcpStream,
    id: Token,
    interest: EventSet
}

impl TCPLink {
    fn read<'a,'b>(&mut self, processor: &'a mut core::processor::Processor<'b>) -> Option<(core::packet::message::Message, Vec<usize>)> {
        loop {
            let mut buf = [0; 4096];
            match self.socket.try_read(&mut buf) {
                Err(e) => {
                    println!("Error while reading socket: {:?}", e);
                    break;
                },
                Ok(None) => {
                    break;
                },
                Ok(Some(len)) => {
                    if len > 0 {
                        let msg = core::packet::decode_packet(&buf[0..len]);
                        match processor.process_message(msg, self.id.as_usize()) {
                            Ok((Some(msg), id)) => { // content, return it
                                return Some((msg, id));
                            },
                            Ok((None, _)) => {
                                break;
                            }
                            Err(e) => {
                                println!("Error: {:?}", e);
                                break;
                            }
                        }
                    } else {
                        break
                    }
                }
            };
        };
        return None;
    }

    fn write(&mut self, msg: core::packet::message::Message) {
        println!("Attempting to write a message to this socket");
        let bytes: Vec<u8> = msg.bytes();
        self.socket.write(&bytes[..]);
    }

    // fn new(token: Token, socket: TcpStream, processor: &'a mut core::processor::Processor<'a>) -> TCPLink<'a> {
    fn new(token: Token, socket: TcpStream) -> TCPLink {
        TCPLink {
            id: token,
            socket: socket,
            interest: EventSet::readable()
        }
    }
}

struct LinkManager<'a,'b : 'a> {
    processor: &'a mut core::processor::Processor<'b>,
    links: HashMap<Token, TCPLink>, // TOOD: make this a generic type of Link
    link_names: HashMap<String, Token>,
    token_counter: usize,
    tcp_socket: TcpListener,
    udp_socket: UdpSocket,
    ctl_socket: UdpSocket,
}

const SERVER_TCP_TOKEN: Token = Token(0);
const SERVER_UDP_TOKEN: Token = Token(1);
const SERVER_CTL_TOKEN: Token = Token(2);
const SERVER_BASE_TOKEN: Token = Token(3);

impl<'a,'b> LinkManager<'a,'b> {
    // TODO: invoke this later...
    fn emit_message(&mut self, msg: core::packet::message::Message, token: Token) {
        // self.links.get(&token).unwrap().write(msg);
    }

    fn new(tcp_socket: TcpListener, udp_socket: UdpSocket, ctl_socket: UdpSocket,
        processor: &'a mut core::processor::Processor<'b>) -> LinkManager<'a,'b> {
        LinkManager {
            processor: processor,
            links: HashMap::new(),
            link_names: HashMap::new(),
            tcp_socket: tcp_socket,
            udp_socket: udp_socket,
            ctl_socket: ctl_socket,
            token_counter: SERVER_BASE_TOKEN.as_usize(),
        }
    }

    fn get_link_id_by_name(&mut self, nick: &String) -> Option<&mut Token> {
        return self.link_names.get_mut(nick);
    }

    fn get_link_by_id(&mut self, token: &Token) -> Option<&mut TCPLink> {
        // if self.links.contains_key(token) {
            return self.links.get_mut(token);
        // }
        // return None;
    }

    pub fn process_control(&mut self, event_loop: &mut EventLoop<LinkManager>, msg: String) -> Result<bool, ControlMessageError> {
        let mut params: Vec<&str> = msg.trim().split(" ").collect();

        let mut cmd = params[0];
        if cmd == "mk" {
            let mut target = params[1];
            if target == "link" {
                let mut nick = params[2];
                let mut protocol = params[3];
                let mut address = params[4];

                println!("{} {}", protocol, address);
                if protocol == "udp" {
                    // self.link_manager.add_link()
                    return Err(ControlMessageError::InvalidControlMessage);
                } else {
                    println!("Adding the link");
                    // self.link_manager.add_link(nick.to_owned(), LinkType::TCP, hostport.to_owned());
                    self.add_link(event_loop, nick.to_owned(), LinkType::TCP, address.to_owned());
                    return Ok(false);
                }
            } else if target == "route" {
                // mk route linkname <route ~~ lci:/foo/bar
                let mut name = params[2];
                let mut route = params[3];

                let link_id = self.link_names.get_mut(&name.to_owned()).unwrap();
                // let link_id = Token(10);
                self.processor.add_fib_entry(route.to_owned(), link_id.as_usize());

                return Ok(false);
            } else if target == "listener" {
                return Err(ControlMessageError::InvalidControlMessage);
            }
        }

        return Err(ControlMessageError::InvalidControlMessage);
    }

    fn add_link(&mut self, event_loop: &mut EventLoop<LinkManager>, linkNick: String, linktype: LinkType, address: String)  {
        // TODO: this should match on the link type
        self.token_counter += 1;
        let new_token = Token(self.token_counter);

        print!("New link added with ID {}", new_token.as_usize());

        let socket_address = address.parse::<SocketAddr>().unwrap();
        match TcpStream::connect(&socket_address).unwrap() {
            socket => {
                let link = TCPLink::new(new_token.clone(), socket);
                self.links.insert(new_token, link);
                event_loop.register(&self.links[&new_token].socket,
                                        new_token, EventSet::readable(),
                                        PollOpt::edge() | PollOpt::oneshot()).unwrap();
            }
        }

        self.link_names.insert(linkNick, new_token);
    }
}

impl<'a,'b> Handler for LinkManager<'a,'b> {
    type Timeout = usize;
    type Message = (Token, core::packet::message::Message);

    fn ready(&mut self, event_loop: &mut EventLoop<LinkManager>, token: Token, events: EventSet) {
        if events.is_readable() {
            match token {
                SERVER_TCP_TOKEN => {
                    let client_socket = match self.tcp_socket.accept() {
                        Err(e) => {
                            println!("Accept error: {}", e);
                            return;
                        },
                        Ok(None) => unreachable!("Accept has returned 'None'"),
                        Ok(Some((sock, addr))) => sock
                    };

                    self.token_counter += 1;
                    let new_token = Token(self.token_counter);

                    println!("Spawning a new TCP link with token {}", new_token.as_usize());

                    let link = TCPLink::new(new_token.clone(), client_socket);
                    self.links.insert(new_token, link);
                    event_loop.register(&self.links[&new_token].socket,
                                            new_token, EventSet::readable(),
                                            PollOpt::edge() | PollOpt::oneshot()).unwrap();
                },
                SERVER_UDP_TOKEN => {
                    let mut buf = [0; 4096]; // 4k MTU for UDP, by default
                    match self.udp_socket.recv_from(&mut buf) {
                        Err(e) => {
                            // TODO
                        },
                        Ok(None) => {
                            unreachable!("recv_from has returned 'None'");
                        },
                        Ok(Some((amt,src))) => {
                            println!("Got a new connection message"); // FOR EACH PACKET, THIS IS CALLED
                            // TODO: process the packet.
                        }
                    }
                },
                SERVER_CTL_TOKEN => {
                    // let buf = RingBuf::new(8192); // 8K MTU max
                    let mut buf = [0; 4096]; // 4k MTU for UDP, by default
                    let (cnt, _) = unsafe {
                        self.ctl_socket.recv_from(&mut buf).unwrap().unwrap()
                    };

                    println!("cnt = {}", cnt);

                    let mut byte_vector = Vec::new();
                    for i in 0..cnt {
                        byte_vector.push(buf[i]);
                    }
                    let command_string = String::from_utf8(byte_vector).unwrap();

                    println!("Received: {}", command_string);

                    event_loop.reregister(&self.udp_socket, SERVER_CTL_TOKEN, EventSet::readable(),
                                          PollOpt::edge() | PollOpt::oneshot()).unwrap();

                    match self.process_control(event_loop, command_string.clone()) {
                        Ok(true) => {
                            println!("Done! {}", command_string.clone());
                            println!("Success, and pass this command to the forwarder.");
                        }, Ok(false) => {
                            println!("Success, but take no further action.");
                        }, Err(e) => {
                            println!("Error parsing the command.");
                        }
                    }
                },
                token => {
                    let mut ingress_link = self.links.get_mut(&token).unwrap();

                    match ingress_link.read(self.processor) {
                        Some((msg, ids)) => {
                            for id in ids {
                                println!("FORWARD TO ID {}", id);
                                let token = Token(id as usize);

                                let sender = event_loop.channel();
                                //sender.send((token, msg));
                            }
                        }
                        None => {
                            // pass
                        }
                    };

                    event_loop.reregister(&ingress_link.socket, token, EventSet::readable(),
                                          PollOpt::edge() | PollOpt::oneshot()).unwrap();
                }
            }
        }
    }

    // WRITE
    fn notify(&mut self, event_loop: &mut EventLoop<LinkManager>, msg_token: (Token, core::packet::message::Message)) {
        print!("Finally... sending the message to the outbound link.");
        let (token, msg) = msg_token;
        let mut egress_link = self.links.get_mut(&token).unwrap();
        egress_link.write(msg);
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
    let mut processor = core::processor::Processor::new(fwd);

    let default_tcp_listener = "127.0.0.1:9696".parse::<SocketAddr>().unwrap();
    let server_tcp_socket = TcpListener::bind(&default_tcp_listener).unwrap();

    let default_udp_listener = "127.0.0.1:9697".parse::<SocketAddr>().unwrap();
    let server_udp_socket = UdpSocket::bound(&default_udp_listener).unwrap();

    let default_ctl_listener = "127.0.0.1:9698".parse::<SocketAddr>().unwrap();
    let server_ctl_socket = UdpSocket::bound(&default_ctl_listener).unwrap();

    let mut event_loop = EventLoop::new().unwrap();
    event_loop.register(&server_tcp_socket,
        Token(0),
        EventSet::readable(),
        PollOpt::edge()).unwrap();

    event_loop.register(&server_udp_socket,
        Token(1),
        EventSet::readable(),
        PollOpt::edge()).unwrap();

    event_loop.register(&server_ctl_socket,
        Token(2),
        EventSet::readable(),
        PollOpt::edge()).unwrap();

    let mut handler = LinkManager::new(server_tcp_socket, server_udp_socket, server_ctl_socket, &mut processor);

    event_loop.run(&mut handler).unwrap();
}
