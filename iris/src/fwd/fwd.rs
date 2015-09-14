use fwd::fib as fib;
use fwd::pit as pit;
use fwd::cs as cs;

struct Forwarder {
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
