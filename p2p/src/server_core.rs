use super::connection_manager;
use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use std::{thread};

// STATE_INIT = 1
// STATE_STANDBY = 2
// STATE_CONNECTED_TO_NETWORK = 3
// STATE_SHUTTING_DOWN = 4

#[derive(Debug)]
pub struct ServerCore {
    server_state: u8,
    addr: SocketAddr,
    parent_addr: SocketAddr,
    cm: connection_manager::ConnectionManager
}

impl ServerCore {
    pub fn new(addr: SocketAddr, parent_addr: SocketAddr) -> ServerCore {
        let cm = connection_manager::ConnectionManager::new(addr);
        let server = ServerCore { 
            server_state: 1,
            addr: addr,
            parent_addr: parent_addr,
            cm: cm
        };
        println!("Server IP address is set to ...{}", addr.ip());
        return server;
    }

    pub fn start(&mut self) {
        self.server_state = 2;
        self.cm.start();
    }

    pub fn join_network(&mut self) -> Result<(), failure::Error> {
        println!("hogehogehogehoge");
        let parent_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(000, 0, 0, 0)), 000);
        if self.parent_addr != parent_addr {
            self.server_state = 3;
            self.cm.join_network(self.parent_addr).unwrap();
        } else {
            println!("This server is running as Genesis Core Node ...");
        }
        return Ok(())
    }

    pub fn shutdown(&mut self) {
        self.server_state = 4;
        println!("Shutdown server...");
        self.cm.connection_close().unwrap();
    }

    pub fn get_my_current_state(&self) -> u8 {
        return self.server_state;
    }
}

fn main() {
    // let mut _server = ServerCore::new();
    // &_server.start();
    // &_server.join_network();
    // &_server.shutdown();
    // println!("state: {}", _server.get_my_current_state());
}
