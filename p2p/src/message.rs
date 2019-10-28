use serde::{Deserialize, Serialize};
// use serde_json::Result;
use serde_json::{ json, Value };

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

#[derive(Serialize, Deserialize)]
pub struct Message {
    protocol: String,
    version: String,
    msg_type: u8,
    my_port: u32,
    payload: Vec<String>,
} 

// pub fn build(msg_type: u8, my_port: u32, payload: &Vec<String>) -> Result<Value, failure::Error>{

//     let data = json! ({
//         "protocol": "bthereum".to_string(),
//         "version": "0.1.0".to_string(),
//         "msg_type": msg_type,
//         "my_port": my_port,
//         "payload": payload.to_vec()
//     });

//     Ok(data)
// }

// pub fn parse(msg: Value) -> (String, u8, u8, u32, Vec<String>) {
//     let msg: Message = serde_json::from_value(msg).unwrap();
//     let ver: String = msg.version;
//     let cmd: u8 = msg.msg_type;
//     let my_port: u32 = msg.my_port;
//     let payload: Vec<String> = msg.payload;

//     let error = "error".to_string();
//     let ok = "ok".to_string();

//     if msg.protocol != "bthereum".to_string() {
//         return (error, 1, 8, my_port, payload);
//     } else if ver != "0.1.0".to_string() {
//         return (error, 2, 8, my_port, payload);
//     } else if cmd == 2 {
//         return (ok, 3, cmd, my_port, payload);
//     } else {
//         return (ok, 4, cmd, my_port, payload);
//     }
// }

pub fn build(msg_type: u8, my_port: u32, payload: &Vec<String>) -> Result<Value, failure::Error>{

    let data = json! ({
        "protocol": "bthereum".to_string(),
        "version": "0.1.0".to_string(),
        "msg_type": msg_type,
        "my_port": my_port,
        "payload": payload.to_vec()
    });

    Ok(data)
}

pub fn parse(msg: &Value) -> (String, u8, u8, u32, Vec<String>) {
    let msg: Message = serde_json::from_value(msg.clone()).unwrap();
    let ver: String = msg.version;
    let cmd: u8 = msg.msg_type;
    let my_port: u32 = msg.my_port;
    let payload: Vec<String> = msg.payload;

    let error = "error".to_string();
    let ok = "ok".to_string();

    if msg.protocol != "bthereum".to_string() {
        return (error, 1, 8, my_port, payload);
    } else if ver != "0.1.0".to_string() {
        return (error, 2, 8, my_port, payload);
    } else if cmd == 2 {
        return (ok, 3, cmd, my_port, payload);
    } else {
        return (ok, 4, cmd, my_port, payload);
    }
}

fn main() {
    let mut vec: Vec<String> = Vec::new();
    vec.push("aaa".to_string());
    vec.push("bbb".to_string());
    let data = build(1, 33332, &vec).unwrap();
    let (result, reason, cmd, my_port, payload) = parse(&data);
    println!("result: {}, reason: {}, cmd: {}, myport: {}, payload: {:?}", result, reason, cmd, my_port, payload);
}

