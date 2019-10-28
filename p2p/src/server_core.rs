// STATE_INIT = 0
// STATE_STANDBY = 1
// STATE_CONNECTED_TO_NETWORK = 2
// STATE_SHUTTING_DOWN = 3

#[derive(Debug)]
struct ServerCore {
    state: u8
}

impl ServerCore {
    fn new() -> ServerCore {
        let server = ServerCore { state: 0 };
        return server;
    }

    fn start(&mut self) {
        self.state = 1;
    }

    fn join_network(&mut self) {
        self.state = 2;
    }

    fn shutdown(&mut self) {
        self.state = 3;
        println!("Shutdown server...");
    }

    fn get_my_current_state(self) -> u8 {
        return self.state;
    }
}

fn main() {
    let mut _server = ServerCore::new();
    &_server.start();
    &_server.join_network();
    &_server.shutdown();
    println!("state: {}", _server.get_my_current_state());
}
