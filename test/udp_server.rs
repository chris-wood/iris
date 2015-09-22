use std::net::UdpSocket;
use std::thread;

fn main()
{
    let attempt = UdpSocket::bind("localhost:9596");
    let mut socket;
    match attempt {
        Ok(sock) => {
            println!("Created the socket");
            socket = sock;
        },
        Err(err) => {
            panic!("Unable to bind to localhost:9596");
        }
    }

    let mut buf = [0; 256];
    let receiveAttempt = socket.recv_from(&mut buf);
    match(receiveAttempt) {
        Ok((amt, src)) => {
            println!("Received {}", amt);
        },
        Err(err) => {
            panic!("Failed to receive content.");
        }
    }

    drop(socket); // close the socket
}
