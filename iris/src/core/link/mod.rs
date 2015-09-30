use std::vec;
use common::name;
use std::net::UdpSocket;

trait Face {
    fn send_to(&self) -> book;
}

pub struct FaceContainer {
    faceId: u32,
    // socket: UdpSocket,
    // TODO: need to somehow store the recipient address here, so it can be used in socket.send_to(..)
}

impl FaceContainer {
    pub fn new(id: u32, address: String) -> (Face) {
        Face {
            faceId: id,
            // socket: try!(UdpSocket::bind("127.0.0.1:34254"))
        }
    }

    pub fn sendTo(self, wire_format: &[u8]) -> (bool) {
        // self.socket.send_to();
        return true;
    }
}
