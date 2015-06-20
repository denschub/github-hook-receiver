extern crate rustc_serialize;

pub fn receive(object: rustc_serialize::json::Json) {
    println!("{:?}", object);
}
