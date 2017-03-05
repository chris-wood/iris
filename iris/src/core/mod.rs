pub mod datastructure;
pub mod packet;
pub mod processor;
pub mod link;

use std::vec::Vec as Vec;

use core::datastructure::fib as fib;
use core::datastructure::pit as pit;
use core::datastructure::cs as cs;

use core::packet::Packet as Packet;
use common::name::Name as Name;

#[derive(Debug)]
pub enum ForwarderResult {
    CacheHit,
    PitHit,
    ForwardPacket
}

#[derive(Debug)]
pub enum ForwarderResponseResult {
    ForwardPacket,
}

#[derive(Debug)]
pub enum ForwarderError {
    NoRouteInFib,
    NoMatchingPITEntry,
}

pub struct Forwarder<'a> {
    pit: &'a mut pit::PIT,
    cs: &'a mut cs::Cache,
    fib: &'a mut fib::FIB
}

impl<'a> Forwarder<'a> {
    pub fn new(fcs: &'a mut cs::Cache, fpit: &'a mut pit::PIT, ffib: &'a mut fib::FIB) -> Forwarder<'a> {
        Forwarder {
            cs: fcs,
            pit: fpit,
            fib: ffib
        }
    }

    pub fn add_route(&mut self, prefix: &Name, link_id: usize) {
        self.fib.insert(prefix, link_id);
    }

    fn process_response(&mut self, msg: &Packet, incoming_face: usize) -> Result<(ForwarderResponseResult, Vec<usize>), ForwarderError> {
        let faces: Vec<usize>;
        let request: &Packet;

        match self.pit.lookup(msg) {
            Some((entry, index)) => {
                faces = entry.get_faces();
            },
            None => {
                return Err(ForwarderError::NoMatchingPITEntry);
            }
        };

        // let request = entry.
        // XXX: need to insert into the cache, and verify in the process

        self.pit.flush(msg);

        return Ok((ForwarderResponseResult::ForwardPacket, faces));
    }

    fn process_interest<'b>(&mut self, msg: &'b Packet, incoming_face: usize) -> Result<(ForwarderResult, Option<&'b Packet>, Vec<usize>), ForwarderError> {
        let cs = &self.cs;
        let cs_match = match cs.lookup(msg) {
            Some(entry) => {
                // XXX: need to return the actual cached result: entry.build_message()
                return Ok((ForwarderResult::CacheHit, Some(msg), vec!(incoming_face)));
            },
            None => {
                let pit = &mut self.pit;

                let mut to_collapse = false;
                match pit.lookup(msg) {
                    Some(_) => {
                        to_collapse = true;
                    },
                    None => {}
                };

                if to_collapse {
                    pit.insert(&msg, incoming_face);
                    return Ok((ForwarderResult::PitHit, None, Vec::new()));
                }

                let fib = &self.fib;
                match fib.lookup(msg) {
                    Some(entry) => {
                        pit.insert(&msg, incoming_face);
                        return Ok((ForwarderResult::ForwardPacket, Some(msg), entry.faces.clone()));
                    },
                    None => {
                        return Err(ForwarderError::NoRouteInFib);
                    }
                };
            },
        };
    }
}
