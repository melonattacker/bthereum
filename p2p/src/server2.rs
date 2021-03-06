use super::server_core;
use std::net::{SocketAddr};
use std::{thread};
use std::sync::mpsc;

extern crate signal_hook;

use std::io::Error;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

pub fn main(my_addr: SocketAddr, parent_addr: SocketAddr) -> Result<(), Error> {
    let mut server: server_core::ServerCore = server_core::ServerCore::new(my_addr, parent_addr);

    let local_server = server.inner.clone();

    let term = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::SIGINT, Arc::clone(&term))?;

    thread::spawn(move || {
        server.start();
    });   
    
    local_server.lock().unwrap().join_network().unwrap();

    while !term.load(Ordering::Relaxed) {
    }

    local_server.lock().unwrap().shutdown();

    // handle.join().unwrap();
    Ok(())
}
