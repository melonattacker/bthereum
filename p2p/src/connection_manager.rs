use std::io::{Read};
use std::net::{TcpListener, TcpStream, SocketAddr, IpAddr, Ipv4Addr};
use std::{str, thread};
use serde_json::{Value};
use std::sync::Arc;

mod message;

// const PROTOCOL_NAME: &'static str = "bthereum";
// const VERSION: &'static str = "0.1.0";

// const MSG_ADD: u8 = 1;
// const MSG_REMOVE: u8 = 2;
// const MSG_CORE_LIST: u8 = 3;
// const MSG_REQUEST_CORE_LIST: u8 = 4;
// const MSG_PING: u8 = 5;
// const MSG_ADD_EDGE: u8 = 6;
// const MSG_REMOVE_EDGE: u8 = 7;
// const NULL = 8;

// const ERR_PROTOCOL_UNMATCH: u8 = 1;
// const ERR_VERSION_UNMATCH: u8 = 2;
// const OK_WITH_PAYLOAD: u8 = 3;
// const OK_WITHOUT_PAYLOAD: u8 = 4;

#[derive(Debug)]
struct ChildConnectionManager {
    addr: SocketAddr,
    core_node_set: Vec<SocketAddr>
}
#[derive(Debug)]
struct ConnectionManager {
    inner: Arc<ChildConnectionManager>
}

impl ConnectionManager {
    fn new(addr: SocketAddr) -> ConnectionManager {
        println!("Initializing ConnectionManager...");
        let manager = ConnectionManager {
            inner: Arc::new(
                ChildConnectionManager {
                    addr: addr,
                    core_node_set: Vec::new()
                }
            )
        };
        return manager;
    }

    fn start(self) {
    }

    fn send_msg(self) {
    }

    fn join_network(self) {
    }

    fn send_msg_to_all_peers(self) {
    }

    fn _wait_for_access(&mut self) -> Result<(), failure::Error> {
        let mut _local_self = self.inner.clone();
        let listener = TcpListener::bind(_local_self.addr).unwrap();
        loop {
            let mut local_self = self.inner.clone();
            println!("Waiting for the connection...");
            let (stream, addr) = listener.accept()?;
            println!("Connected by... {}", addr);
            thread::spawn(move|| {
                local_self._handle_message(stream, addr).unwrap();
            });
        }
    }
}

impl ChildConnectionManager {
    fn _handle_message(&self, mut stream: TcpStream, addr: SocketAddr) -> Result<(), failure::Error> {
        let mut buffer = [0u8; 1024];
        loop {
            let nbytes = stream.read(&mut buffer)?;
            if nbytes == 0 {
                println!("Connection closed.");
                return Ok(());
            }
            // print!("{}", str::from_utf8(&buffer[..nbytes])?);
            let data: &str = str::from_utf8(&buffer[..nbytes])?;
            let message: Value = serde_json::from_str(data).unwrap();
            let (result, reason, cmd, my_port, payload) = message::parse(&message);
            println!("result: {}, reason: {}, cmd: {}, my_port: {}, payload: {:?}", result, reason, cmd, my_port, payload);
            let status = (result, reason);
            if status == ("error".to_string(), 1) {
                println!("Error: Protocol name is not matched");
                return Ok(());
            }
            else if status == ("error".to_string(), 2) {
                println!("Error: Protocol version is not matched");
                return Ok(());
            }
            else if status ==("ok".to_string(), 4) {
                if cmd == 1 {
                    println!("Add node request was received!!");
                    // self._add_peer(addr);
                    return Ok(());
                }
            }
        }
    }

    fn _add_peer(&mut self, peer: SocketAddr) -> Result<(), failure::Error> {
        println!("Adding peer: {}", peer);
        self.core_node_set.push(peer);
        return Ok(())
    }

    fn _remove_peer(&mut self, peer: SocketAddr) -> Result<(), failure::Error> {
        if self.core_node_set.contains(&peer) {
            for i in 0..self.core_node_set.len() {
                if self.core_node_set[i] == peer {
                    println!("Removing peer: {}", peer);
                    self.core_node_set.remove(i);
                    println!("Current Core List: {:?}", self.core_node_set);
                }
            }
        }
        return Ok(())
    }

    fn _check_peers_connection(self) {
    }
}
fn main() {
    let socket: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 33333);
    let mut manager = ConnectionManager::new(socket);
    // let socket2: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 33331);
    // let manager2 = manager._add_peer(socket2);
    &manager._wait_for_access();
    // manager2._remove_peer(socket2)
}