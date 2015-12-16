use std::sync::mpsc::Receiver;
use std::sync::mpsc;

use core;
use core::packet;
use core::Forwarder as Forwarder;
use core::packet::message::Message as Message;

pub struct Processor<'a> {
    fwd: Forwarder<'a>,
    queue: Receiver<(Message, u16)>
}

impl<'a> Processor<'a> {
    pub fn new(fwdRef: Forwarder<'a>, channel: Receiver<(Message, u16)>) -> Processor {
        Processor {
            fwd: fwdRef,
            queue: channel
        }
    }

    pub fn run(&mut self) {
        loop {
            let attempt = self.queue.recv();
            match attempt {
                Ok((msg, face)) => {
                    println!("Processing a message!");
                    self.process_message(msg, face);
                },
                Err(e) => {
                    println!("Error receiving a message");
                }
            }
        }
    }

    fn process_interest(&mut self, msg: Message, incomingFace: u16) {
        println!("Processing an interest...");
        self.fwd.process_interest(msg, incomingFace);
    }

    fn process_content(&self, msg: Message, incomingFace: u16) {
        println!("Processing a content object.");
        // TODO
    }

    pub fn process_message(&mut self, msg: Message, incomingFace: u16) {
        // TODO: (1) CS, (2) PIT, (3) FIB
        // if (self.fwd.cs.lookup(msg.))
        println!("here we go!");

        if msg.packet_type == core::packet::message::PacketType::Interest {
            self.process_interest(msg, incomingFace);
        } else if msg.packet_type == core::packet::message::PacketType::ContentObject {
            self.process_content(msg, incomingFace);
        } else {
            // TODO: interest return
        }
    }
}
