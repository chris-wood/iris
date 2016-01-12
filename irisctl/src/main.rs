extern crate mio;

use mio::*;

use std::path::Path;
use std::env;
use std::io;
use std::io::prelude::*;
use std::str;

use std::collections::HashMap;
use std::net::SocketAddr;
use mio::udp::*;

fn control_repl(socket: UdpSocket, target: &SocketAddr) {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        write!(&mut stdout, "> ");
        stdout.flush();

        let mut input = String::new();
        stdin.read_line(&mut input);

        let mut params: Vec<&str> = input.trim().split(" ").collect();
        let mut cmd = params[0];

        if cmd == "exit" {
            writeln!(&mut stdout, "\n\tAu revoir :-)\n");
            break;
        } else {
            // TODO: perform validation here
            let mut payload = String::new();
            payload.push_str(&input[0..input.len()]);

            match socket.send_to(payload.as_bytes(), target) {
                Ok(Some(n)) => {
                    // wrote n bytes
                },
                Ok(None) => {
                    println!("Error: nothing was sent");
                },
                Err(e) => {
                    panic!("Error: {}", e)
                }
            }
        }

        writeln!(&mut stdout, "Input: {:?}", input);
    }
}

fn main() {
    println!("Hello, iris!");

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("usage: irisctl <address>");
        return;
    }

    let source_address_string = &args[1];
    let target_address_string = &args[2];

    let udp_address = source_address_string.parse::<SocketAddr>().unwrap();
    let ctl_socket = UdpSocket::bound(&udp_address).unwrap();
    let target_address = target_address_string.parse::<SocketAddr>().unwrap();

    control_repl(ctl_socket, &target_address);
}


// API and commands to support
// - mk key
// - mk dev PARAMS // local and remove device, e.g., eth0, tcp0, udp0, etc.
// - mk service SERVICEID // protocol version
// - mk link LOCAL-DEVICE REMOTE-DEVICE
// - mk pipe LOCAL-SERVICE REMOTE-SERVICE
