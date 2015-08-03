use std::process::Command;
use std::str;

use repo_config::*;

use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha1::Sha1;

use rustc_serialize::hex::ToHex;
use rustc_serialize::json::*;

#[derive(Debug)]
pub struct GithubHook {
    pub event: String,
    pub payload: String,
    pub signature: String,
    pub config_root: String
}

impl GithubHook {
    pub fn new(event: &str, payload: &str, signature: &str, config_root: &str) -> GithubHook {
        GithubHook {
            event: event.to_string(),
            payload: payload.to_string(),
            signature: signature.to_string(),
            config_root: config_root.to_string()
        }
    }

    pub fn receive(&self) {
        let json_payload = Json::from_str(&self.payload).unwrap();
        let repo_name = unwrap_json_string(json_payload.find_path(&["repository", "full_name"]));

        info!("Received {} for {}", self.event, repo_name);

        let repo_config_filename = self.config_root.clone() + "/" + &str::replace(&repo_name, "/", "__") + ".json";
        let repo_config = RepoConfig::new(repo_config_filename);

        match self.is_valid(repo_config.secret.as_ref()) {
            false => {
                error!("Payload validation failed, aborting!");
                panic!();
            },
            true => info!("Payload validation succeeded.")
        };

        let event = self.event.as_ref();
        if repo_config.handlers.contains_key(event) {
            let handler = repo_config.handlers.get(event).unwrap();
            match event {
                "push" => self.process_push(json_payload, &repo_config, handler),
                "pull_request" => self.process_pull_request(json_payload, handler),
                _ => self.process_default(handler)
            };
        }
    }

    fn is_valid(&self, secret: Option<&String>) -> bool {
        match secret {
            None => {
                info!("No secret in config file, skipping validation.");
                true
            },
            Some(secret) => {
                let mut hmac = Hmac::new(Sha1::new(), &secret.as_bytes());
                hmac.input(&self.payload.as_bytes()[..]);

                str::replace(&self.signature, "sha1=", "") == hmac.result().code().to_hex()
            }
        }
    }

    fn process_push(&self, json_payload: Json, repo_config: &RepoConfig, handler: &String) {
        let push_ref = unwrap_json_string(json_payload.find("ref"));
        let refs = &repo_config.refs;
        for aref in refs {
            if aref == &push_ref {
                let mut command = Command::new(handler);

                let head = unwrap_json_string(json_payload.find_path(&["head_commit", "id"]));
                command.env("HEAD", head);

                self.execute_handler(&mut command);
                break;
            }
        }
    }

    fn process_pull_request(&self, json_payload: Json, handler: &String) {
        let mut command = Command::new(handler);

        command.env("ACTION", unwrap_json_string(json_payload.find("action")));

        let pr_number = unwrap_json_number(json_payload.find_path(&["pull_request", "number"]));
        command.env("PR", pr_number.to_string());

        let base = unwrap_json_string(json_payload.find_path(&["pull_request", "base", "ref"]));
        command.env("BASE", base.to_string());

        self.execute_handler(&mut command);
    }

    fn process_default(&self, handler: &String) {
        let mut command = Command::new(handler);
        self.execute_handler(&mut command);
    }

    fn execute_handler(&self, handler: &mut Command) {
        match handler.status() {
            Ok(_) => info!("Command ran successfully."),
            Err(exception) => error!("Command failed: {}", exception)
        }
    }
}

fn unwrap_json_string(json: Option<&Json>) -> String {
    json.unwrap().as_string().unwrap().to_string()
}

fn unwrap_json_number(json: Option<&Json>) -> i64 {
    json.unwrap().as_i64().unwrap()
}
