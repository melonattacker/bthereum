use super::server_core;
use std::net::{SocketAddr};
use std::{thread};

extern crate signal_hook;

use std::io::Error;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

pub fn main(my_addr: SocketAddr, parent_addr: SocketAddr) -> Result<(), Error> {

    let server: server_core::ServerCore = server_core::ServerCore::new(my_addr, parent_addr);

    let local_server = server.inner.clone();

    let term = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::SIGINT, Arc::clone(&term))?;

    let handle = thread::spawn(move || {
        let local_server2 = server.inner.clone();
        local_server2.lock().unwrap().start();
    });   

    while !term.load(Ordering::Relaxed) {
    }
    // local_server.lock().unwrap().shutdown();
    // println!("{:#?}", local_server.lock().unwrap().server_state);
    // println!("hogehoge");

    // handle.join().unwrap();
    Ok(())
}