pub mod datastructure;
pub mod link;
pub mod packet;
pub mod processor;

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
    ForwardInterest,
    ForwardContent
}

#[derive(Debug)]
pub enum ForwarderError {
    NoRouteInFib
}

pub struct Forwarder<'a> {
    pit: &'a mut pit::PIT,
    cs: &'a cs::Cache,
    fib: fib::FIB
}

impl<'a> Forwarder<'a> {
    pub fn new(fcs: &'a cs::Cache, fpit: &'a mut pit::PIT, ffib: fib::FIB) -> Forwarder<'a> {
        Forwarder {
            cs: fcs,
            pit: fpit,
            fib: ffib
        }
    }

    pub fn add_route(&mut self, prefix: &Name, link_id: usize) {
        print!("Inserting into FIB: ");

        let mut cloned = prefix.clone();
        cloned.display();
        println!("");

        self.fib.insert(prefix, link_id);
    }

    fn process_response(&mut self, msg: Message, incoming_face: usize) -> Result<(ForwarderResult, Vec<usize>), ForwarderError> {
        let vec = Vec::new();
        return Ok((ForwarderResult::ForwardContent, vec));
    }

    fn process_interest(&mut self, msg: Message, incoming_face: usize) -> Result<(ForwarderResult, Option<Message>, usize), ForwarderError> {
        println!("Processing an interest.");

        // let mut name = msg.get_name();

        // TODO: need to implement the getters for these things
        // let mut key_id_restr = Vec::new();
        // let mut hash_restr = Vec::new();

        let cs = &self.cs;
        cs.dump_contents();

        // let cs_match = match cs.lookup(&name, &key_id_restr, &hash_restr) {
        let cs_match = match cs.lookup(&msg) {
            Some(entry) => {
                println!("In the cache!");
                return Ok((ForwarderResult::CacheHit, Some(msg), incoming_face));
            }, None => {
                println!("Not in the cache!");

                // TODO: lookup the PIT
                let pit = &mut self.pit;

                let mut toinsert = false;
                // let pit_match = match pit.lookup(&name, &key_id_restr, &hash_restr) {
                let pit_match = match pit.lookup(&msg) {
                    Some(entry) => {
                        println!("In the PIT!");
                        return Ok((ForwarderResult::PitHit, None, 0));
                    },
                    None => {
                        println!("Not in the PIT!");

                        // Flag insertion. (Can't do it here because we're borrowing pit from above call to lookup)
                        toinsert = true;

                        // TODO: forward according to the FIB
                        println!("Forward accordingly...");

                        let fib = &self.fib;

                        // let fib_match = match fib.lookup(&name) {
                        let fib_match = match fib.lookup(&msg) {
                            Some(entry) => {
                                println!("In the FIB!");
                                return Ok((ForwarderResult::ForwardInterest, Some(msg), entry.faces[0])); // TODO: this is where some strategy is applied.
                            },
                            None => {
                                println!("No FIB entry--DROP!");
                                return Err(ForwarderError::NoRouteInFib);
                            }
                        };
                    }
                };

                if toinsert {
                    println!("I inserted this into the PIT.");
                    // pit.insert(&name, &key_id_restr, &hash_restr, incoming_face);
                    pit.insert(&msg, incoming_face);
                }
            },
        };
    }
}
