extern crate traceroute;
extern crate clap;

use std::{thread, time, process};
use clap::{Arg, App, ArgMatches};

fn main() {

    let matches = App::new("routechecker")
        .version("0.1.0")
        .author("Francis Begyn")
        .about("Quick program to check if a hop is in a route")
        .arg(Arg::with_name("hop")
             .required(true)
             .help("hop to check for in the route"))
        .arg(Arg::with_name("destination")
             .default_value("8.8.8.8")
             .help("Destination to traceroute"))
        .arg(Arg::with_name("delay")
             .short("d")
             .default_value("5000")
             .help("Delay between each check in milliseconds"))
        .arg(Arg::with_name("port")
             .short("p")
             .default_value("0")
             .help("Port to trace to"))
        .get_matches();
    
    if let Err(e) = run(matches) {
        println!("App error: {}",e);
        process::exit(1);
    }
}

fn run(matches: ArgMatches) -> Result<(), String>{
    let port: &str = matches.value_of("port").expect("Could not read port");

    let hop: &str = matches.value_of("hop").expect("Could not read hop address");
    let hop_port = format!("{}:{}",hop,port);

    let dst: &str = matches.value_of("destination").expect("Could not read dst address");
    let ip = format!("{}:{}",dst,port);

    let millis: u64 = matches.value_of("delay").unwrap().parse().expect("Could not read delay time");

    let hop_addr: std::net::SocketAddr = hop_port.parse().expect("Unable to parse SocketAddr from hop");
    let addr: &str = &ip;
    let mut ok = false;
    let five_seconds = time::Duration::from_millis(millis);
    loop {
        println!("Printing the route:");
        let mut i: u16 = 1;
        for result_ip in traceroute::execute(addr).expect("Traceroute failed") {
            let comp: traceroute::TraceHop = result_ip.expect("Result address not found");
            println!("Hop {:?}: {:?}", i,comp.host);
            if comp.host == hop_addr {
                ok = true;
            }
            i += 1;
        }
        if ok {
            println!("Route is going through: {:?}", hop_addr);
        } else {
            println!("Wrong way!");
        }
        println!();
        thread::sleep(five_seconds)
    }
}
