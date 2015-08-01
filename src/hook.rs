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
        let repo_name = json_payload.find_path(&["repository", "full_name"]).unwrap().as_string().unwrap();

        info!("Received payload for {}", repo_name);

        let repo_config_filename = self.config_root.clone() + "/" + &str::replace(&repo_name, "/", "__") + ".json";
        let repo_config = RepoConfig::new(repo_config_filename);

        match self.is_valid(repo_config.secret) {
            false => {
                error!("Payload validation failed, aborting!");
                panic!();
            },
            true => info!("Payload validation succeeded.")
        };

        let push_ref = json_payload.find("ref").unwrap().as_string().unwrap();
        for aref in &repo_config.refs {
            if aref == push_ref {
                match Command::new(repo_config.command).status() {
                    Ok(_) => info!("Command ran successfully."),
                    Err(exception) => error!("Command failed: {}", exception)
                };
                break;
            }
        }
    }

    fn is_valid(&self, secret: Option<String>) -> bool {
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
}
