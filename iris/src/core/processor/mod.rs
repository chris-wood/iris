use std::sync::mpsc::Receiver;
use std::sync::mpsc;

use core;
use packet;
use core::Forwarder as Forwarder;
use packet::message::Message as Message;

pub struct Processor {
    fwd: Forwarder,
    queue: Receiver<Message>
}
