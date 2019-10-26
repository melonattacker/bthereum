use serde::{Deserialize, Serialize};
// use serde_json::Result;
use serde_json::json;

// const PROTOCOL_NAME: &'static str = "bthereum";
// const VERSION: &'static str = "0.1.0";

// const MSG_ADD: u8 = 1;
// const MSG_REMOVE: u8 = 2;
// const MSG_CORE_LIST: u8 = 3;
// const MSG_REQUEST_CORE_LIST: u8 = 4;
// const MSG_PING: u8 = 5;
// const MSG_ADD_EDGE: u8 = 6;
// const MSG_REMOVE_EDGE: u8 = 7;

// const ERR_PROTOCOL_UNMATCH: u8 = 1;
// const ERR_VERSION_UNMATCH: u8 = 2;
// const OK_WITH_PAYLOAD: u8 = 3;
// const OK_WITHOUT_PAYLOAD: u8 = 4;

#[derive(Serialize, Deserialize)]
pub struct Message {
    protocol: String,
    version: String,
    msg_type: u8,
    payload: Vec<String>,
} 

pub fn build(msg_type: u8, payload: &Vec<String>) -> Result<String, failure::Error>{

    let data = json! ({
        "protocol": "ethereum".to_string(),
        "version": "0.1.0".to_string(),
        "msg_type": msg_type,
        "payload": payload.to_vec()
    });

    let msg: String = data.to_string();

    Ok(msg)
}

pub fn parse(msg: String) {
}

