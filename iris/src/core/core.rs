use core::datastructure::fib as fib;
use core::datastructure::pit as pit;
use core::datastructure::cs as cs;

use common;
use packet;

pub struct Forwarder {
    cs: cs::Cache,
    pit: pit::PIT,
    fib: fib::FIB
}

impl Forwarder {
    pub fn new() -> Forwarder {
        Forwarder {
            cs: cs::Cache::new(),
            pit: pit::PIT::new(),
            fib: fib::FIB::new()
        }
    }
}