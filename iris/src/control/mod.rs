use std::io;
use std::io::prelude::*;

use core;
use core::processor;

use std::net::UdpSocket;
use std::net::TcpStream;
use std::net::TcpListener;
use std::net::SocketAddr;
use std::net::SocketAddrV4;

use std::sync::mpsc::{Sender,Receiver};
use std::sync::mpsc;

use std::thread;
use std::thread::JoinHandle;

use core::link::LinkListener;
use core::link::LinkListenerError;

fn show_usage() {
    println!("Commands to execute:");
    println!("   mk link <protocol> <name> <address> [port]");
    println!("   mk listener <protocol> <name> [port]");
    println!("   mk route <link name> <route prefix> [cost]");

    // rm link <linkname>
    // show <links/routes>
    // mk route <linkname> lci:/<path>
    // rm route <linkname> lci:/<path>
    // exit
}

pub enum ControlListenerError {
    CouldNotSpawnThread,
    CouldNotCloneSocket
}

pub struct ControlListener {
    address: SocketAddrV4,
    socket: UdpSocket,
    channel: Sender<String>
}

impl ControlListener {
    pub fn new(sender: Sender<String>, addr: SocketAddrV4) -> Box<LinkListener> {
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

        let listener = ControlListener {
            address: addr,
            socket: socket,
            channel: sender
        };
        return Box::new(listener);
    }
}

impl LinkListener for ControlListener {
    fn listen(&self) -> Result<JoinHandle<()>, LinkListenerError>{
        match self.socket.try_clone() {
            Ok(socket) => {
                let channel = self.channel.clone();
                let listenerThread = thread::spawn(move || {

                    // TODO: move this to a separate function

                    println!("Inner control loop listener");
                    let mut buf = [0; 4096]; // 4k MTU for UDP, by default
                    loop {
                        match socket.recv_from(&mut buf) {
                            Ok((amt, src)) => {
                                println!("Okay, got a control message -- sending it now.");
                                let mut byte_vector = Vec::new();
                                for i in 0..amt {
                                    byte_vector.push(buf[i]);
                                }
                                let command_string = match String::from_utf8(byte_vector) {
                                    Ok(v) => v,
                                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                                };

                                // let mut params: Vec<String> = command_string.trim().split(" ").collect();
                                channel.send(command_string);
                            },
                            Err(e) => {
                                println!("couldn't recieve a datagram: {}", e);
                            }
                        }
                    };
                });
                return Ok(listenerThread);
            },
            Err(e) => {
                return Err(LinkListenerError::CouldNotSpawnThread);
            }
        }
        return Err(LinkListenerError::CouldNotCloneSocket);
    }
}

pub fn control_repl(processor: &core::processor::Processor) {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        write!(&mut stdout, "> ");
        stdout.flush();

        let mut input = String::new();
        stdin.read_line(&mut input);

        let mut params: Vec<&str> = input.trim().split(" ").collect();
        let mut cmd = params[0];

        if cmd == "mk" {
            command_mk(params[1..].to_vec());
        } else if cmd == "exit" {
            break;
        }

        writeln!(&mut stdout, "Input: {:?}", input);
    }

        // match io::stdin().read_line(&mut input) {
        //     Ok(n) => {
        //         if !str::is_empty(&input) {
        //             let mut params: Vec<String> = input.trim().split(" ").collect();
        //             let mut cmd = params[0];
        //             // println!("GOT: {}", input);
        //             if cmd == "mk" {
        //                 // TODO: need to slice off the head (take the tail)
        //                 command_mk(params[1..].to_vec());
        //             } else {
        //                 // TODO: parse the others
        //             }
        //         }
        //         input = String::new();
        //     }
        //     Err(error) => println!("error: {}", error)
        // }


        // let input = str::trim(raw_input);
        // if input[0] == ':' as u8 {
        //     let command = str::slice(input, 1, str::len(input));
        //     run_colon_command(command);
        // } else {
        //     rsess = match do task::try |copy rsess| {
        //         run_input(input, rsess, os::args()[0])
        //     } {
        //         result::Ok(s) => copy s,
        //         result::Err(_) => move rsess,
        //     };
        // }
    // }
}

fn command_mk(params: Vec<&str>) -> (bool) {
    let mut target = params[0];

    if target == "link" {
        // TODO
    } else if target == "route" {
        // TODO
    } else if target == "listener" {
        // TODO
    }

    return false;
}

fn command_rm(params: Vec<String>) -> (bool) {
    return false;
}

fn command_get(params: Vec<String>) -> (bool) {
    return false;
}

fn command_set(params: Vec<String>) -> (bool) {
    return false;
}

// API and commands to support
// - mk key
// - mk dev PARAMS // local and remove device, e.g., eth0, tcp0, udp0, etc.
// - mk service SERVICEID // protocol version
// - mk link LOCAL-DEVICE REMOTE-DEVICE
// - mk pipe LOCAL-SERVICE REMOTE-SERVICE
