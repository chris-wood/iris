extern crate mio;

use mio::*; // http://nbaksalyar.github.io/2015/07/10/writing-chat-in-rust.html

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

use std::collections::HashMap;
use std::net::SocketAddr;
use mio::tcp::*;
use mio::udp::*;

fn setup_listeners(control_sender: Sender<String>, sender: Sender<(core::packet::message::Message, u16)>) -> (Vec<Box<core::link::LinkListener>>, Vec<JoinHandle<()>>) {
    let mut listen_handles = Vec::new();
    let mut listeners = Vec::new();

    // Default control listener interface
    // let localhost = Ipv4Addr::new(127,0,0,1);
    // let default_ctl_addr = std::net::SocketAddrV4::new(localhost, 9698);
    // let default_ctl_listener = control::ControlListener::new(control_sender.clone(), default_ctl_addr);
    // let listen_result = default_ctl_listener.listen();
    // match listen_result {
    //     Ok(listener) => {
    //         println!("Created the control listener on 9698");
    //         listen_handles.push(listener);
    //     },
    //     Err(e) => {
    //         println!("Could not listen on port 9698");
    //     }
    // }

    // TODO: bring these back when done with the control setup

    // // Default UDP listener interface
    // let localhost = Ipv4Addr::new(127,0,0,1);
    // let default_udp_addr = std::net::SocketAddrV4::new(localhost, 9696);
    // let default_udp_listener = core::link::UDPLinkListener::new(sender.clone(), default_udp_addr);
    // let listen_result = default_udp_listener.listen();
    // match listen_result {
    //     Ok(listener) => {
    //         println!("Created the UDP listener on 9696");
    //         listen_handles.push(listener);
    //     },
    //     Err(e) => {
    //         println!("Could not listen on 9696");
    //     }
    // }
    //
    // // Default TCP listener interface
    // let default_tcp_addr = std::net::SocketAddrV4::new(localhost, 9697);
    // let default_tcp_listener = core::link::TCPLinkListener::new(sender.clone(), default_tcp_addr);
    // let listen_result = default_tcp_listener.listen();
    // match listen_result {
    //     Ok(listener) => {
    //         println!("Created the TCP listener on 9697");
    //         listen_handles.push(listener);
    //     },
    //     Err(e) => {
    //         println!("Could not listen on 9697");
    //     }
    // }

    return (listeners, listen_handles);
}

fn run(listeners: Vec<JoinHandle<()>>) {
    println!("Joining on the listeners");
    for listener in listeners {
        listener.join();
    }
}

struct TCPLink<'a> {
    socket: TcpStream,
    // processor: &'a mut core::processor::Processor<'a>,
    fib: &'a mut HashMap<String, Vec<Token>>,
    id: Token
}

impl<'a> TCPLink<'a> {
    fn read(&mut self) {
        loop {
            let mut buf = [0; 4096];
            match self.socket.try_read(&mut buf) {
                Err(e) => {
                    println!("Error while reading socket: {:?}", e);
                    return
                },
                Ok(None) =>
                    // Socket buffer has got no more bytes.
                    break,
                Ok(Some(len)) => {
                    // TODO: decode the message and send it to the thing
                    let msg = core::packet::decode_packet(&buf[0..len]);
                    // self.processor.process_message(msg, self.id.as_usize() as u16);
                    // TODO: get the result and do something with it!
                }
            }
        }
    }

    fn write(&self, msg: core::packet::message::Message) {
        println!("Attempting to write a message to this socket");
        // self.socket.write(msg.bytes())
    }

    // fn new(token: Token, socket: TcpStream, processor: &'a mut core::processor::Processor<'a>) -> TCPLink<'a> {
    fn new(token: Token, socket: TcpStream, fib: &'a mut HashMap<String, Vec<Token>>) -> TCPLink<'a> {
        TCPLink {
            id: token,
            socket: socket,
            // processor: processor
            fib: fib
        }
    }
}

struct LinkManager<'a> {
    processor: &'a mut core::processor::Processor<'a>,
    links: HashMap<Token, TCPLink<'a>>, // TOOD: make this a generic type of Link
    token_counter: usize,
    tcp_socket: TcpListener,
    udp_socket: UdpSocket,
    ctl_socket: UdpSocket,

    // TODO: ADD OTHER STUFF HERE
    fib: HashMap<String, Vec<Token>>
}

const SERVER_TCP_TOKEN: Token = Token(0);
const SERVER_UDP_TOKEN: Token = Token(1);
const SERVER_CTL_TOKEN: Token = Token(2);

impl<'a> LinkManager<'a> {
    // TODO: invoke this later...
    fn emit_message(&mut self, msg: core::packet::message::Message, token: Token) {
        self.links.get(&token).unwrap().write(msg);
    }

    fn new(tcp_socket: TcpListener, udp_socket: UdpSocket, ctl_socket: UdpSocket,
        processor: &'a mut core::processor::Processor<'a>) -> LinkManager {
        LinkManager {
            processor: processor,
            links: HashMap::new(),
            tcp_socket: tcp_socket,
            udp_socket: udp_socket,
            ctl_socket: ctl_socket,
            token_counter: 3,
            fib: HashMap::new()
        }
    }
}

impl<'a> Handler for LinkManager<'a> {
    type Timeout = usize;
    type Message = ();

    fn ready(&mut self, event_loop: &mut EventLoop<LinkManager>, token: Token, events: EventSet) {
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

                println!("Spawning a new TCP link");

                self.token_counter += 1;
                let new_token = Token(self.token_counter);

                // CAW: this is the problem.
                // let processor = self.processor;
                let link = TCPLink::new(new_token.clone(), client_socket, &mut self.fib);

                // self.links.insert(new_token, link);
                // event_loop.register(&self.links[&new_token].socket,
                //                         new_token, EventSet::readable(),
                //                         PollOpt::edge() | PollOpt::oneshot()).unwrap();
            },
            SERVER_UDP_TOKEN => {
                let mut buf = [0; 4096]; // 4k MTU for UDP, by default
                match self.udp_socket.recv_from(&mut buf) {
                    Err(e) => {
                        // TODO
                    },
                    Ok(None) => {
                        unreachable!("Accept has returned 'None'");
                    },
                    Ok(Some((amt,src))) => {
                        println!("Got a new connection message"); // FOR EACH PACKET THIS IS CALLED
                        // TODO: do something with this...
                    }
                }
            },
            SERVER_CTL_TOKEN => {
                // TODO
            }
            token => {
                let mut link = self.links.get_mut(&token).unwrap();
                link.read();
                event_loop.reregister(&link.socket, token, EventSet::readable(),
                                      PollOpt::edge() | PollOpt::oneshot()).unwrap();
            }
        }
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
    // let (itx, irx): (Sender<(core::packet::message::Message, u16)>, Receiver<(core::packet::message::Message, u16)>) = mpsc::channel(); // for receiving messages
    // let (otx, orx): (Sender<(core::packet::message::Message, u16)>, Receiver<(core::packet::message::Message, u16)>) = mpsc::channel(); // for sending messages
    // let (ctrl_tx, ctrl_rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    // let (listeners, handlers) = setup_listeners(ctrl_tx, itx.clone());

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

    // let mut handler = LinkManager {
    //     // input_channel: itx,
    //     // output_channel: orx,
    //     processor: &mut processor,
    //     token_counter: 3,        // Starting the token counter from 3 (0 1 2 are the tcp,udp,ctl tokens)
    //     links: HashMap::new(),   // Creating an empty HashMap
    //     tcp_socket: server_tcp_socket,    // Handling the ownership of the socket to the struct
    //     udp_socket: server_udp_socket,
    //     ctl_socket: server_ctl_socket
    // };

    let mut handler = LinkManager::new(server_tcp_socket, server_udp_socket, server_ctl_socket, &mut processor);

    event_loop.run(&mut handler).unwrap();


    // let link_table = core::link::LinkTable::new();
    // let link_manager = core::link::LinkManager::new(itx, orx, link_table, listeners);
    //
    // // TODO: need to put the output receiver queue in the link table, tie the listeners up to the link table, and then implement the logic in the link table to send information to the links
    //
    // // Create the processor to handle the link <-> core message passing
    // // Move the receiver into the processor
    // let mut processor = core::processor::Processor::new(fwd, irx, otx, ctrl_rx, link_manager);
    // processor.run();
    //
    // run(handlers);
}


//////// DEAD CODE BELOW

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
