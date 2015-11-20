use std::sync::mpsc::Receiver;
use std::sync::mpsc;

use core;
use core::packet;
use core::Forwarder as Forwarder;
use core::packet::message::Message as Message;

pub struct Processor {
    fwd: Forwarder,
    queue: Receiver<Message>
}

impl Processor {
    pub fn new(fwdRef: Forwarder, channel: Receiver<Message>) -> Processor {
        Processor {
            fwd: fwdRef,
            queue: channel
        }
    }

    fn process_interest(&self, msg: Message) {
        println!("Processing an interest.")
        // TODO
    }

    fn process_content(&self, msg: Message) {
        println!("Processing a content object.")
        // TODO
    }

    pub fn process_message(&self, msg: Message) {
        // TODO: (1) CS, (2) PIT, (3) FIB
        // if (self.fwd.cs.lookup(msg.))
        println!("here we go!");

        if msg.packet_type == core::packet::message::PacketType::Interest {
            self.process_interest(msg);
        } else if msg.packet_type == core::packet::message::PacketType::ContentObject {
            self.process_content(msg);
        } else {
            // TODO: interest return
        }
    }
}
