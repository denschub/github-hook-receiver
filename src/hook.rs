use std::fs::File;
use std::io::Read;
use std::process::Command;
use std::str;

use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha1::Sha1;
use rustc_serialize::hex::ToHex;
use rustc_serialize::json::*;
use rustc_serialize::json;

#[derive(Debug)]
pub struct GithubHook {
    pub payload: String,
    pub signature: String
}

#[derive(Debug, RustcDecodable)]
pub struct RepoConfig {
    command: String,
    refs: Vec<String>,
    secret: Option<String>
}

pub fn receive(hook: GithubHook, config_root: String) {
    let json_payload = Json::from_str(&hook.payload).unwrap();
    let repo_name = json_payload.find_path(&["repository", "full_name"]).unwrap().as_string().unwrap();

    let repo_config_filename = config_root + "/" + &str::replace(&repo_name, "/", "__") + ".json";
    let repo_config = load_repo_config(repo_config_filename);

    match repo_config.secret {
        None => {},
        Some(secret) => {
            match is_valid(secret.into_bytes(), hook.payload.into_bytes(), hook.signature) {
                false => { panic!("Incoming hook payload is not valid!"); },
                true => {}
            };
        }
    };

    let push_ref = json_payload.find("ref").unwrap().as_string().unwrap();
    for aref in &repo_config.refs {
        if aref == push_ref {
            match Command::new(repo_config.command).status() {
                Ok(_) => {},
                Err(exception) => {
                    println!("{}", exception);
                }
            };
            break;
        }
    }
}

fn is_valid(secret: Vec<u8>, payload: Vec<u8>, signature: String) -> bool {
    let raw_signature = str::replace(&signature, "sha1=", "");
    let mut hmac = Hmac::new(Sha1::new(), &secret);

    hmac.input(&payload[..]);
    let result = hmac.result().code().to_hex();

    raw_signature == result
}

fn load_repo_config(repo_config_filename: String) -> RepoConfig {
    let mut file = match File::open(&repo_config_filename) {
        Ok(file) => file,
        Err(err) => { panic!("Could not load config file at '{}': {}", repo_config_filename, err); }
    };

    let mut file_contents = String::new();
    match file.read_to_string(&mut file_contents) {
        Ok(_) => {},
        Err(err) => { panic!("Could not read config file at '{}: {}'", repo_config_filename, err); }
    };

    json::decode(&file_contents).unwrap()
}
