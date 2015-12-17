pub mod datastructure;
pub mod link;
pub mod packet;
pub mod processor;

use core::datastructure::fib as fib;
use core::datastructure::pit as pit;
use core::datastructure::cs as cs;

use core::packet::message::Message as Message;
use common::name::Name as Name;

use common;

pub enum ForwarderResult {
    CacheHit,
    PitHit,
    ForwardInterest
}

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

    fn process_interest(&mut self, msg: Message, incomingFace: u16) -> Result<(ForwarderResult, Option<Message>, u16), ForwarderError> {
        println!("Processing an interest.");

        let mut name = msg.get_name();

        // TODO: need to implement the getters for these things
        let mut key_id_restr = Vec::new();
        let mut hash_restr = Vec::new();

        let cs = &self.cs;
        cs.dump_contents();

        let cs_match = match cs.lookup(&name, &key_id_restr, &hash_restr) {
            Some(entry) => {
                println!("In the cache!");
                return Ok((ForwarderResult::CacheHit, Some(msg), incomingFace));
            }, None => {
                println!("Not in the cache!");

                // TODO: lookup the PIT
                let pit = &mut self.pit;

                let mut toinsert = false;
                let pit_match = match pit.lookup(&name, &key_id_restr, &hash_restr) {
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
                        let fib_match = match fib.lookup(&name) {
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
                    pit.insert(&name, &key_id_restr, &hash_restr, incomingFace);
                }
            },
        };
    }
}
