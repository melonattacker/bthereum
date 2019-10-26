// STATE_INIT = 0
// STATE_STANDBY = 1
// STATE_CONNECTED_TO_NETWORK = 2
// STATE_SHUTTING_DOWN = 3


struct ServerCore {
    state: u8
}

impl ServerCore {
    // fn new() -> u8 {
    //     ServerCore { state: 0 };
    // }

    fn start(mut self) -> ServerCore {
        self.state = 1;
        return self;
    }

    fn join_network(mut self) -> ServerCore {
        self.state = 2;
        return self;
    }

    fn shutdown(mut self) {
        self.state = 3;
        println!("Shutdown server...");
    }

    fn get_my_current_state(self) -> u8 {
        return self.state;
    }
}

fn main() {
    let mut _server = ServerCore { state: 0 };
    // let state = &_server.start();
    let mut server2 = _server.start();
    let mut server3 = server2.join_network();
    println!("state: {}", server3.get_my_current_state());
}
