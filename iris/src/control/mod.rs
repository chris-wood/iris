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
