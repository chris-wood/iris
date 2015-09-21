use std::net::UdpSocket;
use std::thread;

fn main()
{
    let attempt = UdpSocket::bind("127.0.0.1:9695");
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

    //let mut buf = [0; 10];
    //let (amt, src) = socket.recv_from(&mut buf).unwrap();

    let raw = [1; 10];
    let buf = &raw[..10];

    let result = socket.send_to(buf, "localhost:9999");
    match result {
        Ok(bytes) => {
            println!("Sent {} bytes", bytes);
        },
        Err(err) => {
            panic!("Failed to send to localhost:9999");
        }
    }
    
    thread::sleep_ms(3000);

    drop(socket); // close the socket
}

