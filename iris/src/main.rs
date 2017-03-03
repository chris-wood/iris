extern crate mio;
extern crate bytes;

use std::str;

mod common;
mod control;
mod core;

use core::datastructure::fib as fib;
use core::datastructure::pit as pit;
use core::datastructure::cs as cs;
use core::link;

use mio::*;
use mio::tcp::TcpListener;
use mio::udp::UdpSocket;

use std::net::{SocketAddr};


fn main() {
    // Create the default data structures
    let mut fcs = cs::Cache::new(0);
    let mut fpit = pit::PIT::new();
    let mut ffib = fib::FIB::new();

    // Create the forwarder and message processor
    // NOTE: the forwarder now owns all three structures, and the processor owns the forwarder.
    let fwd = core::Forwarder::new(&mut fcs, &mut fpit, &mut ffib);
    let mut processor = core::processor::Processor::new(fwd);

    // TODO(cawood): only process addresses here -- defer the construction of sockets to the manager
    // TODO(cawood): rename LinkManager to Carrier or NIC or something..

    let default_tcp_listener = "127.0.0.1:9696".parse::<SocketAddr>().unwrap();
    let server_tcp_socket = TcpListener::bind(&default_tcp_listener).unwrap();

    let default_udp_listener = "127.0.0.1:9697".parse::<SocketAddr>().unwrap();
    let server_udp_socket = UdpSocket::bind(&default_udp_listener).unwrap();

    let default_ctl_listener = "127.0.0.1:9698".parse::<SocketAddr>().unwrap();
    let server_ctl_socket = UdpSocket::bind(&default_ctl_listener).unwrap();

    let poll = Poll::new().unwrap();
    poll.register(&server_tcp_socket, link::SERVER_TCP_TOKEN, Ready::readable(), PollOpt::edge()).unwrap();
    poll.register(&server_udp_socket, link::SERVER_UDP_TOKEN, Ready::readable(), PollOpt::edge()).unwrap();
    poll.register(&server_ctl_socket, link::SERVER_CTL_TOKEN, Ready::readable(), PollOpt::edge()).unwrap();

    // let mut event_loop = EventLoop::new().unwrap();
    // event_loop.register(&server_tcp_socket,
    //     Token(0),
    //     EventSet::readable(),
    //     PollOpt::edge()).unwrap();
    //
    // event_loop.register(&server_udp_socket,
    //     Token(1),
    //     EventSet::readable(),
    //     PollOpt::edge()).unwrap();
    //
    // event_loop.register(&server_ctl_socket,
    //     Token(2),
    //     EventSet::readable(),
    //     PollOpt::edge()).unwrap();

    // let mut handler = link::LinkManager::new(server_tcp_socket, server_udp_socket, server_ctl_socket, &mut processor);
    // event_loop.run(&mut handler).unwrap();

    let mut handler = link::LinkManager::new(poll, server_tcp_socket, server_udp_socket, server_ctl_socket, &mut processor);
    handler.service()
}
