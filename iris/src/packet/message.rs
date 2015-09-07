enum MessageType {
    Interest,
    ContentObject,
    InterestReturn
}

struct Message {
    message_type: MessageType,
    // TODO: what else?
}
