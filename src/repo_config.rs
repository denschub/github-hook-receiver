use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

use rustc_serialize::json::*;
use rustc_serialize::json;

#[derive(Debug, RustcDecodable)]
pub struct RepoConfig {
    pub handlers: HashMap<String, String>,
    pub refs: Vec<String>,
    pub secret: Option<String>
}

impl RepoConfig {
    pub fn new(repo_config_filename: String) -> RepoConfig {
        let mut file = match File::open(&repo_config_filename) {
            Ok(file) => {
                info!("Config file loaded.");
                file
            },
            Err(err) => {
                error!("Could not load config file at '{}': {}", repo_config_filename, err);
                panic!();
            }
        };

        let mut file_contents = String::new();
        match file.read_to_string(&mut file_contents) {
            Ok(_) => info!("Config file read."),
            Err(err) => {
                error!("Could not read config file at '{}: {}'", repo_config_filename, err);
                panic!()
            }
        };

        json::decode(&file_contents).unwrap()
    }
}
