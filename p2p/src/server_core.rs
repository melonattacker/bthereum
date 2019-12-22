use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, SocketAddr, IpAddr, Ipv4Addr, Shutdown};
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::{str, thread, time::Duration};
use serde_json::{Value};

use super::message;

// STATE_INIT = 1
// STATE_STANDBY = 2
// STATE_CONNECTED_TO_NETWORK = 3
// STATE_SHUTTING_DOWN = 4

#[derive(Debug)]
pub struct ChildServerCore {
    pub server_state: u8,
    pub addr: SocketAddr,
    pub parent_addr: SocketAddr,
    pub core_node_set: Vec<SocketAddr>,
    pub is_running: Arc<AtomicBool>
}

#[derive(Debug)]
pub struct ServerCore {
    pub inner: Arc<Mutex<ChildServerCore>>,
}

impl ServerCore {
    pub fn new(addr: SocketAddr, parent_addr: SocketAddr) -> ServerCore {
        let server = ServerCore { 
            inner: Arc::new(
                Mutex::new(
                    ChildServerCore {
                        server_state: 1,
                        addr: addr,
                        parent_addr: parent_addr,
                        core_node_set: Vec::new(),
                        is_running: Arc::new(AtomicBool::new(false))
                    }
                )
            ),
        };
        println!("Server IP address is set to ...{}", addr.ip());
        let local_self = server.inner.clone();
        local_self.lock().unwrap()._add_peer(addr).unwrap();
        return server;
    }

    pub fn start(&mut self) {
        let local_self = self.inner.clone();
        local_self.lock().unwrap().server_state = 2;
        let is_running = Arc::clone(&local_self.lock().unwrap().is_running);
        // is_runningをtrueに設定する
        is_running.store(true, Ordering::Relaxed);
        self._wait_for_access(is_running);
    }

    fn _wait_for_access(&mut self, is_running: Arc<AtomicBool>) {
        let _local_self = self.inner.clone();
        let listener = TcpListener::bind(_local_self.lock().unwrap().addr).unwrap();
        while _local_self.lock().unwrap().is_running.load(Ordering::Relaxed) {
            let local_self = self.inner.clone();
            println!("Waiting for the connection...");
            let (stream, addr) = listener.accept().unwrap();
            println!("Connected by... {}", addr);
            let handle = thread::spawn(move|| {
                local_self.lock().unwrap()._handle_message(stream, addr).unwrap();
            });
            handle.join().unwrap();
        }
    }
}

impl ChildServerCore {
    pub fn join_network(&mut self) -> Result<(), failure::Error> {
        let parent_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(000, 0, 0, 0)), 000);
        if self.parent_addr != parent_addr {
            self.server_state = 3;
            self.connect_to_network(self.parent_addr).unwrap();
        } else {
            println!("This server is running as Genesis Core Node ...");
        }
        return Ok(())
    }

    fn connect_to_network(&mut self, address: SocketAddr) -> Result<(), failure::Error> {
        self.parent_addr = address;
        self._connect_to_p2pnw(address).unwrap();
        return Ok(());
    }

    fn _connect_to_p2pnw(&mut self, address: SocketAddr) -> Result<(), failure::Error> {
        let mut stream = TcpStream::connect(address)?;
        let temp_vec: Vec<SocketAddr> = Vec::new();
        // MSG_ADD
        let msg = message::build(1, self.addr.port(), &temp_vec).unwrap();
        let string: String = msg.to_string();
        stream.write_all(string.as_bytes())?;
        stream.shutdown(Shutdown::Both).expect("shutdown call failed");
        return Ok(());
    }

    fn _handle_message(&mut self, mut stream: TcpStream, addr: SocketAddr) -> Result<(), failure::Error> {
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
            let (result, reason, cmd, peer_port, payload) = message::parse(&message);
            // ホストとメッセージ受付用のポートをがっちゃんこ
            let connect_addr = SocketAddr::new(addr.ip(), peer_port);
            println!("result: {}, reason: {}, cmd: {}, peer_port: {}, payload: {:?}", result, reason, cmd, peer_port, payload);
            let status = (result, reason);
            // ERR_PROTOCOL_UNMATCH
            if status == ("error".to_string(), 1) {
                println!("Error: Protocol name is not matched");
                return Ok(());
            }
            // ERR_VERSION_UNMATCH
            else if status == ("error".to_string(), 2) {
                println!("Error: Protocol version is not matched");
                return Ok(());
            }
            // OK_WITHOUT_PAYLOAD
            else if status == ("ok".to_string(), 4) {
                // MSG_ADD
                if cmd == 1 {
                    println!("ADD node request was received!!");
                    self._add_peer(connect_addr).unwrap();
                    if addr == self.addr {
                        return Ok(());
                    }
                    else {
                        // MSG_CORE_LIST
                        let msg = message::build(3, addr.port(), &self.core_node_set).unwrap();
                        self.send_msg_to_all_peers(&msg);
                        return Ok(())
                    }
                }
                // MSG_REMOVE
                else if cmd == 2 {
                    println!("REMOVE node request was received!! from: {}", addr);
                    self._remove_peer(connect_addr).unwrap();
                    // MSG_CORE_LIST
                    let msg = message::build(3, addr.port(), &self.core_node_set).unwrap();
                    self.send_msg_to_all_peers(&msg);
                    return Ok(())
                }
                // MSG_PING
                else if cmd == 5 {
                    return Ok(())
                }
                // MSG_REQUEST_CORE_LIST
                else if cmd == 4 {
                    println!("List for Core nodes was requested!!");
                    // MSG_CORE_LIST
                    let msg = message::build(3, addr.port(), &self.core_node_set).unwrap();
                    let socket_address = SocketAddr::new(addr.ip(), peer_port);
                    let result = self.send_msg(socket_address, &msg).unwrap();
                    if result != () {
                        self._remove_peer(socket_address).unwrap();
                    }
                    return Ok(())
                }
                else {
                    println!("received unknown command: {}", cmd);
                    return Ok(());
                }
            }
            // OK_WITH_PAYLOAD
            else if status == ("ok".to_string(), 3) {
                // MSG_CORE_LIST
                if cmd == 3 {
                    // 受信したリストを上書きしてるのでセキュアではない
                    println!("Refresh the core node list...");
                    let new_core_set = payload;
                    println!("latest core node list:{:?}", new_core_set);
                    self.core_node_set = new_core_set;
                    return Ok(())
                }
                else {
                    println!("received unknown command: {}", cmd);
                    return Ok(())
                }
            }
            else {
                println!("Unexpected status: {}, {}", status.0, status.1);
            }
        }
    }

    pub fn shutdown(&mut self) {
        println!("Shutdown server...");
        self.is_running.store(false, Ordering::Relaxed);
        self.server_state = 4;
        self.connection_close().unwrap();
    }

    fn connection_close(&mut self) -> Result<(), failure::Error> {
        let stream = TcpStream::connect(self.addr)?;
        stream.shutdown(Shutdown::Both).expect("shutdown call failed");
        let temp_vec: Vec<SocketAddr> = Vec::new();
        // MSG_REMOVE
        let msg = message::build(2, self.addr.port(), &temp_vec).unwrap();
        let result = self.send_msg(self.parent_addr, &msg).unwrap();
        if result != () {
            self._remove_peer(self.parent_addr).unwrap();
        }
        return Ok(());
    }

    fn _add_peer(&mut self, peer: SocketAddr) -> Result<(), failure::Error> {
        println!("Adding peer: ({})", peer);
        self.core_node_set.push(peer);
        println!("Current Core List: {:?}", self.core_node_set);
        return Ok(())
    }

    fn _remove_peer(&mut self, peer: SocketAddr) -> Result<(), failure::Error> {
        if self.core_node_set.contains(&peer) {
            for i in 0..self.core_node_set.len() {
                if self.core_node_set[i] == peer {
                    println!("Removing peer: ({})", peer);
                    self.core_node_set.remove(i);
                    println!("Current Core List: {:?}", self.core_node_set);
                }
            }
        }
        return Ok(())
    }

    fn _check_peers_connection(&mut self) {
        println!("check_peers_connection was called");
        let mut changed: bool = false;
        let mut dead_core_node_set: Vec<SocketAddr> = Vec::new();
        for i in 0..self.core_node_set.len() {
            let result = self._is_alive(self.core_node_set[i]).unwrap();
            if result != true {
                dead_core_node_set.push(self.core_node_set[i]);
                self.core_node_set.remove(i);
            }
        }
        if dead_core_node_set.len() != 0 {
            changed = true;
            println!("Removed {:#?}", dead_core_node_set);
        }
        println!("current core node list: {:#?}", self.core_node_set);

        if changed {
            // MSG_CORE_LIST 
            let msg = message::build(3, self.addr.port(), &self.core_node_set).unwrap();
            self.send_msg_to_all_peers(&msg);
        }
        // 定期的に_check_peers_connectionを呼び出し
        thread::sleep(Duration::from_secs(1800));
        self._check_peers_connection();
    }

    fn _is_alive(&mut self, target: SocketAddr) -> Result<bool, failure::Error> {
        let mut stream = TcpStream::connect(target)?;
        // MSG_PING
        let temp_port: u16 = 0;
        let temp_vec: Vec<SocketAddr> = Vec::new();
        let msg = message::build(5, temp_port, &temp_vec).unwrap();
        let string: &str = msg.as_str().unwrap();
        stream.write_all(string.as_bytes())?;
        stream.shutdown(Shutdown::Both).expect("shutdown call failed");
        return Ok(true);
    }

    fn send_msg_to_all_peers(&mut self, msg: &Value) {
        println!("send_msg_to_all_peers was called!");
        for i in 0..self.core_node_set.len() {
            if self.core_node_set[i] != self.addr {
                println!("message will be sent to ...{}", self.core_node_set[i]);
                let result = self.send_msg(self.core_node_set[i], &msg).unwrap();
                if result != () {
                    self._remove_peer(self.core_node_set[i]).unwrap();
                }
            }
        }
    }

    fn send_msg(&mut self, socket_address: SocketAddr, msg: &Value) -> Result<(), failure::Error> {
        let mut stream = TcpStream::connect(socket_address)?;
        let string: String = msg.to_string();
        stream.write_all(string.as_bytes())?;
        stream.shutdown(Shutdown::Both).expect("shutdown call failed");
        return Ok(())
    }

    fn get_my_current_state(&self) -> u8 {
        return self.server_state;
    }
}
