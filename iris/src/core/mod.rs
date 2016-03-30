pub mod datastructure;
pub mod packet;
pub mod processor;
pub mod link;

use std::vec::Vec as Vec;

use core::datastructure::fib as fib;
use core::datastructure::pit as pit;
use core::datastructure::cs as cs;

use core::packet::message::Message as Message;
use common::name::Name as Name;

use common;

#[derive(Debug)]
pub enum ForwarderResult {
    CacheHit,
    PitHit,
    ForwardMessage
}

#[derive(Debug)]
pub enum ForwarderResponseResult {
    ForwardMessage,
}

#[derive(Debug)]
pub enum ForwarderError {
    NoRouteInFib,
    NoMatchingPITEntry,
    InternalError
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
        let mut cloned = prefix.clone();
        self.fib.insert(prefix, link_id);
    }

    fn process_response(&mut self, msg: &Message, incoming_face: usize) -> Result<(ForwarderResponseResult, Vec<usize>), ForwarderError> {
        let mut do_flush = false;
        let faces: Vec<usize>;

        let pit_match = match self.pit.lookup_mut(msg) {
            Some((entry, index)) => {
                faces = entry.get_faces();
                do_flush = true;
            },
            None => {
                return Err(ForwarderError::NoMatchingPITEntry);
            }
        };

        if do_flush {
            self.pit.flush(msg);
            return Ok((ForwarderResponseResult::ForwardMessage, faces));
        } else {
            return Err(ForwarderError::InternalError);
        }
    }

    fn process_interest<'b>(&mut self, msg: &'b Message, incoming_face: usize) -> Result<(ForwarderResult, Option<&'b Message>, Vec<usize>), ForwarderError> {
        let cs = &self.cs;
        let cs_match = match cs.lookup(msg) {
            Some(entry) => {
                // TODO: this is incorrect.
                let response = entry.build_message();
                return Ok((ForwarderResult::CacheHit, Some(msg), vec!(incoming_face)));
            },
            None => {
                let pit = &mut self.pit;

                let mut to_forward = false;
                let mut to_collapse = false;
                let pit_match = match pit.lookup(msg) {
                    Some(entry) => {
                        to_collapse = true;
                    },
                    None => {
                        to_forward = true;
                    }
                };

                if to_collapse {
                    pit.insert(&msg, incoming_face);
                    return Ok((ForwarderResult::PitHit, None, Vec::new()));
                }

                if to_forward {
                    pit.insert(&msg, incoming_face);

                    let fib = &self.fib;
                    let fib_match = match fib.lookup(msg) {
                        Some(entry) => {
                            return Ok((ForwarderResult::ForwardMessage, Some(msg), entry.faces.clone()));
                        },
                        None => {
                            return Err(ForwarderError::NoRouteInFib);
                        }
                    };
                } else {
                    return Ok((ForwarderResult::PitHit, None, vec!(incoming_face)));
                }
            },
        };
    }
}
