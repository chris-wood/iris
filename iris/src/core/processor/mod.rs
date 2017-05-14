use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;
use std::vec::Vec as Vec;

use core;
use core::packet;
use core::Forwarder as Forwarder;
use core::ForwarderResult as ForwarderResult;
use core::ForwarderResponseResult as ForwarderResponseResult;
use core::packet::Packet as Packet;
use common::name::Name as Name;

pub struct Processor<'a> {
    fwd: Forwarder<'a>,
}

#[derive(Debug)]
pub enum ProcessorError {
    InternalError,
    NotImplementedYet,
    UnsupportedPacketType
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
        let params: Vec<&str> = msg.trim().split(" ").collect();

        let cmd = params[0];
        if cmd == "mk" {
            let target = params[1];
            if target == "link" {
                let nick = params[2];
                let protocol = params[3];
                let hostport = params[4];

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
                let name = params[2];
                let route = params[3];
                let cost = params[4];

                return Err(ProcessorError::NotImplementedYet);
            } else if target == "listener" {
                // TODO
                return Err(ProcessorError::NotImplementedYet);
            }
        }

        return Err(ProcessorError::NotImplementedYet);
    }

    // TODO: extract the send functions to separate functions
    fn process_interest<'b>(&mut self, msg: &'b Packet, incoming_face: usize) -> Result<(Option<&'b Packet>, Vec<usize>), ProcessorError> {
        match self.fwd.process_interest(msg, incoming_face) {
            Ok((ForwarderResult::CacheHit, msg, ids)) => { // content, return it
                // let inner_msg = msg.unwrap();
                // TODO(caw): finishme
                return Err(ProcessorError::NotImplementedYet);
            },
            Ok((ForwarderResult::PitHit, _, _)) => { // do nothing, this is OK
                return Err(ProcessorError::NotImplementedYet);
            },
            Ok ((ForwarderResult::ForwardPacket, msg, ids)) => { // interest, forward it
                return Ok((msg, ids));
            },
            Err(e) => {
                return Err(ProcessorError::NotImplementedYet);
            }
        }
        return Err(ProcessorError::NotImplementedYet);
    }

    fn process_content<'b>(&mut self, msg: &'b Packet, incoming_face: usize) -> Result<(Option<&'b Packet>, Vec<usize>), ProcessorError> {
        match self.fwd.process_response(msg, incoming_face) {
            Ok((ForwarderResponseResult::ForwardPacket, ids)) => {
                return Ok((Some(msg), ids));
            }, Err(e) => {
                return Err(ProcessorError::InternalError);
            }
        }

        return Err(ProcessorError::NotImplementedYet);
    }

    fn process_fragment<'b>(&mut self, msg: &'b Packet, incoming_face: usize) -> Result<(Option<&'b Packet>, Vec<usize>), ProcessorError> {
        // TOOD(cawood): implement the BEF logic here
        // See: https://tools.ietf.org/html/draft-mosko-icnrg-beginendfragment-02

        // Check to see if it's a BEF fragment
        let header: packet::FixedHeader = msg.get_fixed_header();

        
        /*
        if begin, start storing some state
        else if end, end storing state
        else if idle, pass,
        else, middle fragment, so parse and append to the right state bucket
        */

        return Err(ProcessorError::NotImplementedYet);
    }

    pub fn process_message<'b>(&mut self, msg: &'b Packet, incoming_face: usize) -> Result<(Option<&'b Packet>, Vec<usize>), ProcessorError> {
        // TODO: wrap these up in state checker functions
        if msg.get_packet_type() == core::packet::typespace::PacketType::Interest {
            return self.process_interest(msg, incoming_face);
        } else if msg.get_packet_type() == core::packet::typespace::PacketType::ContentObject {
            return self.process_content(msg, incoming_face);
        } else if msg.get_packet_type() == core::packet::typespace::PacketType::Fragment {
            return self.process_fragment(msg, incoming_face);
        } else {
            // TODO: interest return
            return Err(ProcessorError::UnsupportedPacketType)
        }
    }
}
