extern crate crypto;
extern crate rustc_serialize;

use std::str;

use self::crypto::hmac::Hmac;
use self::crypto::mac::Mac;
use self::crypto::sha1::Sha1;
use self::rustc_serialize::hex::ToHex;
use self::rustc_serialize::json::Json;

#[derive(Debug)]
pub struct GithubHook {
    pub payload: String,
    pub signature: String,
}

pub fn receive(hook: GithubHook) {
    // TODO remove me.
    let test_shared_secret: Vec<u8> = "Foo".to_string().into_bytes();

    let json_payload = Json::from_str(&hook.payload).unwrap();

    match is_valid(test_shared_secret, hook.payload.into_bytes(), hook.signature) {
        false => { panic!("Incoming hook payload is not valid!"); },
        true => {}
    };

    println!("{:?}", json_payload);
}

fn is_valid(secret: Vec<u8>, payload: Vec<u8>, signature: String) -> bool {
    let raw_signature = str::replace(&signature, "sha1=", "");
    let mut hmac = Hmac::new(Sha1::new(), &secret);

    hmac.input(&payload[..]);
    let result = hmac.result().code().to_hex();

    raw_signature == result
}
