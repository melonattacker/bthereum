#[macro_use]
extern crate log;
use std::{env, thread};
use std::net::{SocketAddr, IpAddr, Ipv4Addr};
pub mod message;
pub mod connection_manager;
pub mod server_core;
pub mod thread_helper;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        error!("Please specify [server|client] [addr:port].");
        std::process::exit(1);
    }
    let role: &str = &args[1];
    // let address = &args[2];
    if role == "server1" {
        let my_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 33333);
        let parent_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 00000);
        let mut server: server_core::ServerCore = server_core::ServerCore::new(my_address, parent_address);
        server.start();
    } 
    else if role == "server2" {
        let my_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 33334);
        let parent_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 33333);
        let mut server: server_core::ServerCore = server_core::ServerCore::new(my_address, parent_address);
        server.start();
        server.join_network().unwrap();
    }
    else {
        missing_role();
    }
}

fn missing_role() {
    error!("Please specify server or client on the 1st argument.");
    std::process::exit(1);
}
