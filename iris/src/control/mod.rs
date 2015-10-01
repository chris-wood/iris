// TODO: key-store parameter

pub fn control_repl() {

}

fn command_mk(params: Vec<String>) -> (bool) {
    return false;
}

fn command_rm(params: Vec<String>) -> (bool) {
    return false;
}

fn command_get(params: Vec<String>) -> (bool) {
    return false;
}

fn command_set(params: Vec<String>) -> (bool) {
    return false;
}

// API and commands to support
// - mk key
// - mk dev PARAMS // local and remove device, e.g., eth0, tcp0, udp0, etc.
// - mk service SERVICEID // protocol version
// - mk link LOCAL-DEVICE REMOTE-DEVICE
// - mk pipe LOCAL-SERVICE REMOTE-SERVICE
