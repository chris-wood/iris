use std::vec;
use std::thread;
use std::thread::JoinHandle;
use std::io::Write;

use std::net::UdpSocket;
use std::net::TcpStream;
use std::net::TcpListener;
use std::net::SocketAddr;
use std::net::SocketAddrV4;

use common::name;

pub trait Link {
    // TODO: maybe rename run() to receive_from()?
    fn run(&self);
    fn stop(&mut self);
    fn send_to(&mut self, wire_format: &[u8]) -> usize;
}

pub struct UDPLink {
    linkId: u32,
    kill: bool,
    socket: UdpSocket,
    dst: SocketAddr
}

impl UDPLink {
    pub fn new(id: u32, socket: UdpSocket, dst: SocketAddr) -> (Box<Link>) {
        let link = UDPLink {
            linkId: id,
            kill: false,
            socket: socket,
            dst: dst,
        };
        return Box::new(link);
    }
}

pub struct TCPLink {
    linkId: u32,
    kill: bool,
    stream: TcpStream,
    dst: SocketAddr
}

impl TCPLink {
    pub fn new(id: u32, stream: TcpStream, dst: SocketAddr) -> (Box<Link>) {
        let link = TCPLink {
            linkId: id,
            kill: false,
            stream: stream,
            dst: dst
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

    fn run(&self) {
        println!("Inside the the UDP link run() function");
        loop {
            // TODO... listen for data, and then send it to the forwarder
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

    fn run(&self) {
        loop {
            // TODO... listen for data, and then send it to the forwarder
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
    socket: UdpSocket
}

impl UDPLinkListener {
    pub fn new(addr: SocketAddrV4) -> Box<LinkListener> {
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
            socket: socket
        };
        return Box::new(listener);
    }
}

impl LinkListener for UDPLinkListener {
    fn listen(&self) -> Result<JoinHandle<()>, LinkListenerError> {
        let clone = self.socket.try_clone();
        match clone {
            Ok(socket) => {
                let listenerThread = thread::spawn(move || {
                    println!("Inner UDP loop listener");
                    let mut buf = [0; 4096]; // 4k MTU for UDP, by default
                    loop {
                        match socket.recv_from(&mut buf) {
                            Ok((amt, src)) => {
                                println!("Got a message");
                                let clone = socket.try_clone();
                                match clone {
                                    Ok(socket) => {
                                        println!("Cloned OK--starting a link.");
                                        thread::spawn(move || {
                                            print!("start the new UDP link!");
                                            let link = UDPLink::new(0, socket, src);
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
    address: SocketAddrV4
}

impl TCPLinkListener {
    pub fn new(addr: SocketAddrV4) -> Box<LinkListener> {
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
        };
        return Box::new(listener);
    }
}

impl LinkListener for TCPLinkListener {
    fn listen(&self) -> Result<JoinHandle<()>, LinkListenerError> {
        let listener = self.listener.try_clone();
        match listener {
            Ok(realListener) => {
                let listenerThread = thread::spawn(move || {
                    for stream in realListener.incoming() {
                        match stream {
                            Ok(stream) => {
                                let addr = stream.peer_addr().unwrap();
                                thread::spawn(move || {
                                    print!("start the link!");
                                    let link = TCPLink::new(0, stream, addr);
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
