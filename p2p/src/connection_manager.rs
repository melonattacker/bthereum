use std::net::{TcpListener, TcpStream};
use std::{str, thread};

struct ConnectionManager {
    host: String,
    port: u32,
    core_node_set: Vec<u32>
}

impl ConnectionManager {
    fn new(host: String, port: u32) -> ConnectionManager {
        println!("Initializing ConnectionManager...");
        let manager = ConnectionManager {
            host: host,
            port: port,
            core_node_set: Vec::new()
        };
        return manager;
    }

    fn start(self) {
    }

    fn join_network(mut self, peer: u32) -> ConnectionManager {
        println!("Adding peer: {}", peer);
        self.core_node_set.push(peer);
        return self;
    }

    fn send_msg(self) {
    }

    fn send_msg_to_all_peers(self) {
    }

    fn _wait_for_access(self) -> Result<(), failure::Error> {
        let port: String = self.port.to_string();
        let address: &str = &(self.host + port);
        let listener = TcpListener::bind(address)?;
        loop {
            println!("Waiting for the connection...")
            let (stream, addr) = listener.accept()?;
            println!("Connected by... {}", addr);
            thread::spawn(move || {
                _handle_message(stream).unwrap_or_else(|error| error!("{:?}", error));
            });
        }
    }

    fn _handle_message(mut stream: TcpStream) -> Result<(), failure::Error> {
        let mut buffer = [0u8; 1024];
        loop {
            let nbytes = stream.read(&mut buffer)?;
            if nbytes == 0 {
                debug!("Connection closed.");
                return Ok(());
            }
            // print!("{}", str::from_utf8(&buffer[..nbytes])?);
            let data = str::from_utf8(&buffer[..nbytes])?;
        }
    }

    fn _add_peer(self) {
    }

    fn _remove_peer(mut self, peer: u32) {
        if self.core_node_set.contains(&peer) {
            for i in 0..self.core_node_set.len() {
                if self.core_node_set[i] == peer {
                    println!("Removing peer: {}", peer);
                    self.core_node_set.remove(i);
                    println!("Current Core List: {:?}", self.core_node_set);
                }
            }
        }
    }

    fn _check_peers_connection(self) {
    }
}

fn main() {
    let manager = ConnectionManager::new("127.0.0".to_string(), 33333);
    let manager2 = manager.join_network(33331);
    manager2._remove_peer(33331)
}