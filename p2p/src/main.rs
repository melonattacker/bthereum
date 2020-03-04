#[macro_use]
extern crate log;
use std::{env, thread, time};
use std::net::{SocketAddr, IpAddr, Ipv4Addr};
mod message;
mod server_core;
mod server1;
mod server2;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        error!("Please specify [server1|server2]");
        std::process::exit(1);
    }
    let role: &str = &args[2];
    let address = &args[2];
    if role == "server1" {
        let my_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 33333);
        let parent_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 00000);
        server1::main(my_address, parent_address).unwrap();
    } 
    else if role == "server2" {
        let my_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 33334);
        let parent_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 33333);
        server2::main(my_address, parent_address);
    }
    else {
        missing_role();
    }
}

fn missing_role() {
    error!("Please specify server1 or server2 on the 1st argument.");
    std::process::exit(1);
}
