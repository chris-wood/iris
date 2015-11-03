pub mod datastructure;
pub mod link;
pub mod core;
pub mod packet;
pub mod processor;

use core::datastructure::fib as fib;
use core::datastructure::pit as pit;
use core::datastructure::cs as cs;

use common;

pub struct Forwarder {
    cs: cs::Cache,
    pit: pit::PIT,
    fib: fib::FIB
}

impl Forwarder {
    pub fn new() -> Forwarder {
        Forwarder {
            cs: cs::Cache::new(0),
            pit: pit::PIT::new(),
            fib: fib::FIB::new()
        }
    }
}
