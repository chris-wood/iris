use std::io;

fn show_usage() {
    println!("usage: iris [-h] <command>");
    println!("<command> The forwarder command to execute:");
    println!("   mk link <schema>://<authority>[/listener][/<options>][/name=<linkname>]");
        //     <schema> == tcp/...
        //     <authority> == <protocol specific address/port>
        //     <options> == local=<true/false>
        // rm link <linkname>
        // show <links/routes>
        // mk route <linkname> lci:/<path>
        // rm route <linkname> lci:/<path>
        // spawn <port>
        // quit
}

pub fn control_repl() {
    println!("Starting the REPL");
    let stdin = io::stdin();
    println!("Still going");
    let mut input = String::new();
    println!("... and going");
    loop {
        println!("");
        print!("iris> ");
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                if !str::is_empty(&input) {
                    let mut params: Vec<&str> = input.trim().split(" ").collect();
                    let mut cmd = params[0];
                    // println!("GOT: {}", input);
                    if cmd == "mk" {
                        // TODO: need to slice off the head (take the tail)
                        command_mk(params[1..].to_vec());
                    } else {
                        // TODO: parse the others
                    }
                }
                input = String::new();
            }
            Err(error) => println!("error: {}", error)
        }


        // let input = str::trim(raw_input);
        // if input[0] == ':' as u8 {
        //     let command = str::slice(input, 1, str::len(input));
        //     run_colon_command(command);
        // } else {
        //     rsess = match do task::try |copy rsess| {
        //         run_input(input, rsess, os::args()[0])
        //     } {
        //         result::Ok(s) => copy s,
        //         result::Err(_) => move rsess,
        //     };
        // }
    }
}

fn command_mk(params: Vec<&str>) -> (bool) {
    println!("Started here!");
    return false;
}

fn command_rm(params: Vec<&str>) -> (bool) {
    return false;
}

fn command_get(params: Vec<&str>) -> (bool) {
    return false;
}

fn command_set(params: Vec<&str>) -> (bool) {
    return false;
}

// API and commands to support
// - mk key
// - mk dev PARAMS // local and remove device, e.g., eth0, tcp0, udp0, etc.
// - mk service SERVICEID // protocol version
// - mk link LOCAL-DEVICE REMOTE-DEVICE
// - mk pipe LOCAL-SERVICE REMOTE-SERVICE
