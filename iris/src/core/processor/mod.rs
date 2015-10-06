use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

use core;
use core::Forwarder as Forwarder;

pub struct Processor {
    fwd: Forwarder,
    inputQueue
}
