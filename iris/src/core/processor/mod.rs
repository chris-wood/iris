use std::sync::mpsc::Receiver;
use std::sync::mpsc;

use core;
use core::packet;
use core::Forwarder as Forwarder;
use core::packet::message::Message as Message;

pub struct Processor<'a> {
    fwd: &'a Forwarder<'a>,
    queue: Receiver<Message>
}

impl<'a> Processor<'a> {
    pub fn new(fwdRef: &'a Forwarder<'a>, channel: Receiver<Message>) -> Processor<'a> {
        Processor {
            fwd: fwdRef,
            queue: channel
        }
    }

    fn process_interest(&self, msg: Message) {
        println!("Processing an interest.");
        let mut name = msg.get_name();

        //let mut key_id_restr = Vec::new();
        //let mut hash_restr = Vec::new();

        let cs = self.fwd.cs;
        cs.dump_contents();

        let cs_match = match self.fwd.cs_lookup(&name, &key_id_restr, &hash_restr) {
            Some(entry) => println!("In the cache!"),
            None => println!("Not in the cache!"),
        };
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
