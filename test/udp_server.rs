use std::net::UdpSocket;
use std::thread;

fn main()
{
    let attempt = UdpSocket::bind("127.0.0.1:9999");
    let mut socket;
    match attempt {
        Ok(sock) => {
            println!("Created the socket");
            socket = sock;
        },
        Err(err) => {
            panic!("Unable to bind to 127.0.0.1:9999");
        } 
    }

    let mut buf = [0; 10];
    let receiveAttempt = socket.recv_from(&mut buf);
    match(receiveAttempt) {
        Ok((amt, src)) => {
            println!("Received {}", amt);
        },
        Err(err) => {
            panic!("Failed to receive content.");
        }
    }

    thread::sleep_ms(3000);

    drop(socket); // close the socket
}

