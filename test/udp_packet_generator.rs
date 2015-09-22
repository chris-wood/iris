use std::net::UdpSocket;

use std::env;
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

fn main()
{
    let attempt = UdpSocket::bind("localhost:9999");
    let mut socket;
    match attempt {
        Ok(sock) => {
            println!("Created the socket");
            socket = sock;
        },
        Err(err) => {
            panic!("Unable to bind to 127.0.0.1:9695");
        }
    }

    let args: Vec<String> = env::args().collect();

    let path = Path::new(&args[1]);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    let mut file_contents = Vec::new();
    match file.read_to_end(&mut file_contents) {
        Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
        Ok(_) => {}
    }
    let buffer = &file_contents[..]; // take reference to the entire thing (i.e., a slice)

    let result = socket.send_to(buffer, "localhost:9596");
    match result {
        Ok(bytes) => {
            println!("Sent {} bytes", bytes);
        },
        Err(err) => {
            panic!("Failed to send to localhost:9999");
        }
    }

    drop(socket); // close the socket
}
