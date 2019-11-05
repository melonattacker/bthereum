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

    // メインスレッド(join) -> サブスレッド(start)のチャンネル
    // let (sender1, receiver1) = mpsc::channel();

    let handle = thread::spawn(move || {
        let local_server2 = server.inner.clone();
        local_server2.lock().unwrap().start();

        // let val = receiver1.recv().unwrap();
        // println!("send from main thread. {}", val);
    });   
    
    local_server.lock().unwrap().join_network().unwrap();

    while !term.load(Ordering::Relaxed) {
    }

    // sender1.send("hello".to_string()).unwrap();

    // start()のスレッドがMutexをロックしてるのでそれを解除する必要がある
    // 今はデッドロックが起きている
    // local_server.lock().unwrap().shutdown();
    // println!("{:#?}", local_server);
    // println!("hogehoge");

    // handle.join().unwrap();
    Ok(())
}
