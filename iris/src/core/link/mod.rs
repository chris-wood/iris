extern crate mio;
extern crate bytes;

use std::io::prelude::*;
use std::collections::HashMap;
use std::net::SocketAddr;
use mio::tcp::*;
use mio::udp::*;

use mio::*;

use std::str;

use core;
use core::packet;
use core::Forwarder as Forwarder;
use core::ForwarderResult as ForwarderResult;
use core::ForwarderResponseResult as ForwarderResponseResult;

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
    id: Token
    // interest: EventSet
}

impl TCPLink {
    fn read<'a,'b>(&mut self) -> Option<(Box<[u8]>, usize)> {
        loop {
            let mut buf = [0; 4096];
            match self.socket.read(&mut buf) {
                Err(e) => {
                    println!("Error while reading socket: {:?}", e);
                    break;
                },
                Ok(len) => {
                    if len > 0 {
                        return Some((Box::new(buf), len));
                    } else {
                        break
                    }
                }
            };
        };
        return None;
    }

    fn write(&mut self, msg: core::packet::message::Message) {
        let bytes: Vec<u8> = msg.bytes();
        self.socket.write(&bytes[..]);
    }

    fn new(token: Token, socket: TcpStream) -> TCPLink {
        TCPLink {
            id: token,
            socket: socket
            // interest: EventSet::readable()
        }
    }
}

pub struct LinkManager<'a,'b : 'a> {
    processor: &'a mut core::processor::Processor<'b>,
    links: HashMap<Token, TCPLink>, // TODO: make this a generic type of Link
    link_names: HashMap<String, Token>,
    token_counter: usize,
    tcp_socket: TcpListener,
    udp_socket: UdpSocket,
    ctl_socket: UdpSocket,
    poller: mio::Poll,
}

pub const SERVER_TCP_TOKEN: Token = Token(0);
pub const SERVER_UDP_TOKEN: Token = Token(1);
pub const SERVER_CTL_TOKEN: Token = Token(2);
pub const SERVER_BASE_TOKEN: usize = 3;

impl<'a,'b> LinkManager<'a,'b> {
    fn emit_message(&mut self, msg: core::packet::message::Message, token: Token) {
        // self.links.get(&token).unwrap().write(msg);
    }

    pub fn new(poller: mio::Poll, tcp_socket: TcpListener, udp_socket: UdpSocket, ctl_socket: UdpSocket,
        processor: &'a mut core::processor::Processor<'b>) -> LinkManager<'a,'b> {
        // XXX: save the sockets and polls

        LinkManager {
            processor: processor,
            links: HashMap::new(),
            link_names: HashMap::new(),
            tcp_socket: tcp_socket,
            udp_socket: udp_socket,
            ctl_socket: ctl_socket,
            token_counter: SERVER_BASE_TOKEN,
            poller: poller,
        }
    }

    pub fn service(&mut self) {
        let mut events = Events::with_capacity(4096);

        // Run forever
        loop {
            self.poller.poll(&mut events, None).unwrap();

            for event in events.iter() {
                match event.token() {
                    SERVER_TCP_TOKEN => {
                        match self.tcp_socket.accept() {
                            Ok((stream, addr)) => {
                                self.token_counter += 1;
                                let new_token = Token(self.token_counter);
                                let link = TCPLink::new(new_token.clone(), stream);
                                self.links.insert(new_token, link);
                                self.poller.register(&self.links[&new_token].socket, new_token, Ready::readable(), PollOpt::edge());
                            },
                            Err(e) => {},
                        }
                    },
                    SERVER_UDP_TOKEN => {
                        // XXX: move to handle_udp_packet()
                    },
                    SERVER_CTL_TOKEN => {
                        // TODO(cawood): implement a TLS connection
                    },
                    token => {
                        unreachable!()
                        // XXX: move to the token
                        // let mut ingress_link = self.links.get_mut(&token).unwrap();
                        // let link_id = ingress_link.id;
                        //
                        // match ingress_link.read() {
                        //     Some((buffer, length)) => {
                        //         match core::packet::decode_packet(&buffer[0..length]) {
                        //             Ok(msg) => {
                        //                 match self.processor.process_message(&msg, link_id.as_usize()) {
                        //                     Ok((Some(output_message), output_ids)) => { // forward the message to every ID in the list
                        //                         for link_id in output_ids {
                        //                             println!("FORWARD TO ID {}", link_id);
                        //                             let token = Token(link_id as usize);
                        //
                        //                             let sender = event_loop.channel();
                        //                             let msg_copy = output_message.clone();
                        //                             sender.send((token, msg_copy));
                        //                         }
                        //                     },
                        //                     Ok((None, _)) => {
                        //                         // pass
                        //                     }
                        //                     Err(e) => {
                        //                         println!("Error: {:?}", e);
                        //                         // pass
                        //                     }
                        //                 }
                        //             },
                        //             Err(_) => {},
                        //         };
                        //     },
                        //     None => { /* pass */ }
                        // };
                    }
                }
            }
        }
    }

    // fn get_link_id_by_name(&mut self, nick: &String) -> Option<&mut Token> {
    //     return self.link_names.get_mut(nick);
    // }
    //
    // fn get_link_by_id(&mut self, token: &Token) -> Option<&mut TCPLink> {
    //     if self.links.contains_key(token) {
    //         return self.links.get_mut(token);
    //     }
    //     return None;
    // }

    // pub fn process_control(&mut self, event_loop: &mut EventLoop<LinkManager>, msg: String) -> Result<bool, ControlMessageError> {
    //     let params: Vec<&str> = msg.trim().split(" ").collect();
    //
    //     let cmd = params[0];
    //     if cmd == "mk" {
    //         let target = params[1];
    //         if target == "link" {
    //             let nick = params[2];
    //             let address = params[4];
    //
    //             println!("{} {}", protocol, address);
    //             if protocol == "udp" {
    //                 // self.link_manager.add_link()
    //                 return Err(ControlMessageError::InvalidControlMessage);
    //             } else {
    //                 println!("Adding the link with nick {}", nick);
    //                 // self.link_manager.add_link(nick.to_owned(), LinkType::TCP, hostport.to_owned());
    //                 self.add_link(event_loop, nick.to_owned(), LinkType::TCP, address.to_owned());
    //                 return Ok(false);
    //             }
    //         } else if target == "route" {
    //             // mk route linkname <route ~~ lci:/foo/bar
    //             let name = params[2];
    //             let route = params[3];
    //
    //             println!("Adding a route for {} {}", name, route);
    //
    //             match self.link_names.get_mut(&name.to_owned()) {
    //                 Some(link_id) => {
    //                     self.processor.add_fib_entry(route.to_owned(), link_id.as_usize());
    //                 }, None => { println!("Link {} not found.", name); }
    //             }
    //             // let link_id = Token(10);
    //
    //
    //             return Ok(false);
    //         } else if target == "listener" {
    //             return Err(ControlMessageError::InvalidControlMessage);
    //         }
    //     }
    //
    //     return Err(ControlMessageError::InvalidControlMessage);
    // }

    fn add_link(&mut self, link_nick: String, link_type: LinkType, address: String)  {
        self.token_counter += 1;
        let new_token = Token(self.token_counter);

        // TODO: pattern match on the LinkType parameter
        let socket_address = address.parse::<SocketAddr>().unwrap();
        match TcpStream::connect(&socket_address).unwrap() {
            socket => {
                let link = TCPLink::new(new_token.clone(), socket);
                self.links.insert(new_token, link);
                self.poller.register(&self.links[&new_token].socket, new_token, Ready::readable(), PollOpt::edge()).unwrap();
            }
        }

        self.link_names.insert(link_nick, new_token);
    }

    // http://carllerche.github.io/mio/mio/struct.Token.html
    pub fn process(&mut self, event: Event) {
        if event.readiness().is_readable() {

        }
    }
}

// impl<'a,'b> Handler for LinkManager<'a,'b> {
//     type Timeout = usize;
//     type Message = (Token, core::packet::message::Message);
//
//     fn ready(&mut self, event_loop: &mut EventLoop<LinkManager>, token: Token, events: EventSet) {
//         if events.is_readable() {
//             match token {
//                 SERVER_TCP_TOKEN => {
//                     let client_socket = match self.tcp_socket.accept() {
//                         Err(e) => {
//                             println!("Accept error: {}", e);
//                             return;
//                         },
//                         Ok(None) => unreachable!("Accept has returned 'None'"),
//                         Ok(Some((sock, addr))) => sock
//                     };
//
                    // self.token_counter += 1;
                    // let new_token = Token(self.token_counter);
                    //
                    // println!("Spawning a new TCP link with token {}", new_token.as_usize());
                    //
                    // let link = TCPLink::new(new_token.clone(), client_socket);
                    // self.links.insert(new_token, link);
                    // event_loop.register(&self.links[&new_token].socket,
                    //                         new_token, EventSet::readable(),
                    //                         PollOpt::edge() | PollOpt::oneshot()).unwrap();
//                 },
//                 SERVER_UDP_TOKEN => {
//                     let mut buf = [0; 4096]; // 4k MTU for UDP, by default
//                     match self.udp_socket.recv_from(&mut buf) {
//                         Err(e) => {
//                             // TODO
//                         },
//                         Ok(None) => {
//                             unreachable!("recv_from has returned 'None'");
//                         },
//                         Ok(Some((amt,src))) => {
//                             println!("Got a new connection message"); // FOR EACH PACKET, THIS IS CALLED
//                             // TODO: process the packet.
//                         }
//                     }
//                 },
//                 SERVER_CTL_TOKEN => {
//                     // let buf = RingBuf::new(8192); // 8K MTU max
//                     let mut buf = [0; 4096]; // 4k MTU for UDP, by default
//                     let (cnt, _) = unsafe {
//                         self.ctl_socket.recv_from(&mut buf).unwrap().unwrap()
//                     };
//
//                     println!("cnt = {}", cnt);
//
//                     let mut byte_vector = Vec::new();
//                     for i in 0..cnt {
//                         byte_vector.push(buf[i]);
//                     }
//                     let command_string = String::from_utf8(byte_vector).unwrap();
//
//                     println!("Received: {}", command_string);
//
//                     event_loop.reregister(&self.udp_socket, SERVER_CTL_TOKEN, EventSet::readable(),
//                                           PollOpt::edge() | PollOpt::oneshot()).unwrap();
//
//                     match self.process_control(event_loop, command_string.clone()) {
//                         Ok(true) => {
//                             println!("Done! {}", command_string.clone());
//                             println!("Success, and pass this command to the forwarder.");
//                         }, Ok(false) => {
//                             println!("Success, but take no further action.");
//                         }, Err(_) => {
//                             println!("Error parsing the command");
//                         }
//                     }
//                 },
//                 token => {
//                     let mut ingress_link = self.links.get_mut(&token).unwrap();
//                     let link_id = ingress_link.id;
//
//                     match ingress_link.read() {
//                         Some((buffer, length)) => {
//                             match core::packet::decode_packet(&buffer[0..length]) {
//                                 Ok(msg) => {
//                                     match self.processor.process_message(&msg, link_id.as_usize()) {
//                                         Ok((Some(output_message), output_ids)) => { // forward the message to every ID in the list
//                                             for link_id in output_ids {
//                                                 println!("FORWARD TO ID {}", link_id);
//                                                 let token = Token(link_id as usize);
//
//                                                 let sender = event_loop.channel();
//                                                 let msg_copy = output_message.clone();
//                                                 sender.send((token, msg_copy));
//                                             }
//                                         },
//                                         Ok((None, _)) => {
//                                             // pass
//                                         }
//                                         Err(e) => {
//                                             println!("Error: {:?}", e);
//                                             // pass
//                                         }
//                                     }
//                                 },
//                                 Err(_) => {},
//                             };
//                         },
//                         None => { /* pass */ }
//                     };
//
//                     event_loop.reregister(&ingress_link.socket, token, EventSet::readable(),
//                                           PollOpt::edge() | PollOpt::oneshot()).unwrap();
//                 }
//             }
//         }
//     }
//
//     fn notify(&mut self, _: &mut EventLoop<LinkManager>, msg_token: (Token, core::packet::message::Message)) {
//         let (token, msg) = msg_token;
//         let mut egress_link = self.links.get_mut(&token).unwrap();
//         egress_link.write(msg);
//     }
// }
