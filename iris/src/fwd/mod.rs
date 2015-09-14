pub mod fib;
pub mod cs;
pub mod pit;
pub mod face;

use common;

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
