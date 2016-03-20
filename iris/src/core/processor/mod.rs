use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;
use std::vec::Vec as Vec;

use core;
use core::packet;
use core::Forwarder as Forwarder;
use core::ForwarderResult as ForwarderResult;
use core::ForwarderResponseResult as ForwarderResponseResult;
use core::packet::message::Message as Message;
use common::name::Name as Name;


use std::io;
use std::io::prelude::*;
use std::thread;

pub struct Processor<'a> {
    fwd: Forwarder<'a>,
}

#[derive(Debug)]
pub enum ProcessorError {
    InternalError,
    NotImplementedYet,
    UnsupportedMessageType
}

impl<'a> Processor<'a> {
    pub fn new(fwdRef: Forwarder<'a>) -> Processor {
        Processor {
            fwd: fwdRef,
        }
    }

    pub fn add_fib_entry(&mut self, prefix: String, link_id: usize) {
        let name = Name::create_from_string(prefix);
        match name {
            Some(n) => {
                self.fwd.add_route(&n, link_id);
            },
            None => {
                println!("Failed to create a name from the string");
            }
        }
    }

    // mk link <name> <protocol> <host:port>
    // protocol == tcp or udp
    // mk route <name> <lci route> [cost]
    pub fn process_control(&mut self, msg: String) -> Result<bool, ProcessorError> {
        let mut params: Vec<&str> = msg.trim().split(" ").collect();

        let mut cmd = params[0];
        if cmd == "mk" {
            let mut target = params[1];
            if target == "link" {
                let mut nick = params[2];
                let mut protocol = params[3];
                let mut hostport = params[4];

                println!("{} {}", protocol, hostport);
                if protocol == "udp" {
                    // self.link_manager.add_link()
                    return Err(ProcessorError::NotImplementedYet);
                } else {
                    println!("Adding the link");
                    // self.link_manager.add_link(nick.to_owned(), LinkType::TCP, hostport.to_owned());
                    return Err(ProcessorError::NotImplementedYet);
                }
            } else if target == "route" {
                let mut name = params[2];
                let mut route = params[3];
                let mut cost = params[4];

                return Err(ProcessorError::NotImplementedYet);
            } else if target == "listener" {
                // TODO
                return Err(ProcessorError::NotImplementedYet);
            }
        }

        return Err(ProcessorError::NotImplementedYet);
    }

    // TODO: extract the send functions to separate functions
    fn process_interest<'b>(&mut self, msg: &'b Message, incoming_face: usize) -> Result<(Option<&'b Message>, Vec<usize>), ProcessorError> {
        match self.fwd.process_interest(msg, incoming_face) {
            Ok((ForwarderResult::CacheHit, msg, id)) => { // content, return it
                let inner_msg = msg.unwrap();

                // TODO: finishme

                return Err(ProcessorError::NotImplementedYet);
            },
            Ok((ForwarderResult::PitHit, _, _)) => { // do nothing, this is OK
                return Err(ProcessorError::NotImplementedYet);
            },
            Ok ((ForwarderResult::ForwardMessage, msg, id)) => { // interest, forward it
                let mut vec = Vec::new();
                vec.push(id);
                return Ok((msg, vec));
            },
            Err(e) => {
                return Err(ProcessorError::NotImplementedYet);
            }
        }
        return Err(ProcessorError::NotImplementedYet);
    }

    fn process_content<'b>(&mut self, msg: &'b Message, incoming_face: usize) -> Result<(Option<&'b Message>, Vec<usize>), ProcessorError> {
        println!("Processing a content object.");

        match self.fwd.process_response(msg, incoming_face) {
            Ok((ForwarderResponseResult::ForwardMessage, ids)) => {
                return Ok((Some(msg), ids));
            }, Err(e) => {
                return Err(ProcessorError::InternalError);
            }
        }

        return Err(ProcessorError::NotImplementedYet);
    }

    pub fn process_message<'b>(&mut self, msg: &'b Message, incoming_face: usize) -> Result<(Option<&'b Message>, Vec<usize>), ProcessorError> {
        // TODO: wrap these up in state checker functions
        if msg.packet_type == core::packet::message::PacketType::Interest {
            return self.process_interest(msg, incoming_face);
        } else if msg.packet_type == core::packet::message::PacketType::ContentObject {
            return self.process_content(msg, incoming_face);
        } else {
            // TODO: interest return
            return Err(ProcessorError::UnsupportedMessageType)
        }
    }
}
