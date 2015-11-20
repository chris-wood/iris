pub mod datastructure;
pub mod link;
pub mod packet;
pub mod processor;

use core::datastructure::fib as fib;
use core::datastructure::pit as pit;
use core::datastructure::cs as cs;

use common;

pub struct Forwarder<'a> {
    pub cs: &'a cs::Cache,
    pub pit: &'a pit::PIT,
    pub fib: &'a fib::FIB
}

impl<'a> Forwarder<'a> {
    pub fn new(fcs: &'a cs::Cache, fpit: &'a pit::PIT, ffib: &'a fib::FIB) -> Forwarder<'a> {
        Forwarder {
            cs: fcs,
            pit: fpit,
            fib: ffib
        }
    }
}
