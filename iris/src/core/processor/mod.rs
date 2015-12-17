use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;

use core;
use core::packet;
use core::link::Link;
use core::Forwarder as Forwarder;
use core::ForwarderResult as ForwarderResult;
use core::packet::message::Message as Message;

use std::io;
use std::io::prelude::*;
use std::thread;

pub struct Processor<'a> {
    fwd: Forwarder<'a>,
    input_queue: Receiver<(Message, u16)>,
    control_channel: Receiver<String>,
    output_queue: Sender<(Message, u16)>
}

impl<'a> Processor<'a> {
    pub fn new(fwdRef: Forwarder<'a>, input_channel: Receiver<(Message, u16)>, output_channel: Sender<(Message, u16)>, control: Receiver<String>) -> Processor {
        Processor {
            fwd: fwdRef,
            input_queue: input_channel,
            control_channel: control,
            output_queue: output_channel
        }
    }

    pub fn run(&mut self) {
        loop {
            let traffic_attempt = self.input_queue.try_recv();
            match traffic_attempt {
                Ok((msg, face)) => {
                    println!("Processing a message!");
                    self.process_message(msg, face);
                },
                Err(e) => {
                    // println!("Error receiving a message");
                }
            }

            // TODO: this is expensive since we're constantly spilling here...
            // a better approach would be to make Control Messages and Network Messages implement a generic
            // Message trait with isControlMessage and isNetworkMessage functions, and then use those
            // predicates to hand the message off to the right handler
            // .... currently, Control Messages are just strings
            let control_attempt = self.control_channel.try_recv();
            match control_attempt {
                Ok(msg) => {
                    println!("Processing a message!");
                    self.process_control(msg);
                },
                Err(e) => {
                    // println!("Error receiving a control message");
                }
            }
        }
    }

    fn process_control(&mut self, msg: String) {
        println!("Handle this control message!");
    }

    // TODO: extract the send functions to separate functions
    fn process_interest(&mut self, msg: Message, incomingFace: u16) {
        println!("Processing an interest...");
        match self.fwd.process_interest(msg, incomingFace) {
            Ok((ForwarderResult::CacheHit, msg, id)) => { // content, return it
                // forward the message to the ID
                let result = self.output_queue.send((msg, id));
                match result {
                    Ok(m) => {
                        println!("Sent content object back to the link.");
                    },
                    Err(e) => {
                        println!("Error: unable to send message to the link.");
                    }
                }
            },
            Ok((ForwarderResult::PitHit, _, _)) => {
                // do nothing, this is okay.
            },
            Ok ((ForwarderResult::ForwardInterest, msg, id)) => { // interest, forward it
                // forward the message to the ID
                let result = self.output_queue.send((msg, id));
                match result {
                    Ok(m) => {
                        println!("Sent content object back to the link.");
                    },
                    Err(e) => {
                        println!("Error: unable to send message to the link.");
                    }
                }
            },
            Err(e) => {
                println!("Error in the forwarer -- check it out.");
            }
        }
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
