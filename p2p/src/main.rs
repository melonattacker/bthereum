#[macro_use]
extern crate log;
mod message;

fn main() {
    let mut vec: Vec<String> = Vec::new();
    vec.push("aaa".to_string());
    vec.push("bbb".to_string());
    let msg = message::build(1, &vec).unwrap();
    println!("{}", msg);
}
