extern crate bodyparser;
extern crate crypto;
extern crate env_logger;
extern crate iron;
extern crate persistent;
extern crate router;
extern crate rustc_serialize;

#[macro_use] extern crate hyper;
#[macro_use] extern crate log;

mod hook;
mod repo_config;

use std::env;
use std::thread;

use hook::*;

use iron::prelude::*;
use iron::status;
use persistent::Read;
use router::{Router};

header! {(XHubSignature, "X-Hub-Signature") => [String]}
header! {(XGitHubEvent, "X-GitHub-Event") => [String]}

const MAX_BODY_LENGTH: usize = 1024 * 1024 * 10;

fn parse_hook(config_dir_str: &str, req: &mut Request) -> IronResult<Response> {
    let config_dir = config_dir_str.to_string();
    let body = req.get::<bodyparser::Raw>();
    let event = req.headers.get::<XGitHubEvent>().unwrap();

    let empty_sig = &XHubSignature("".to_string());
    let signature = req.headers.get::<XHubSignature>().unwrap_or(empty_sig);

    match body {
        Ok(Some(body)) => {
            let hook = GithubHook::new(&event, &body, &signature);
            thread::spawn(move || {
                hook::receive(hook, config_dir);
            });
            Ok(Response::with(status::Ok))
        },
        _ => Ok(Response::with(status::NotFound))
    }
}

fn main() {
    env_logger::init().unwrap();

    let args: Vec<_> = env::args().collect();
    if args.len() < 2 || args.len() > 3 {
        println!("Usage: github-hook-receiver <config dir (no trailing slash.)> [<listen address (127.0.0.1:3000)>]");
        std::process::exit(1);
    }

    let config_dir = args.get(1).unwrap().to_string();
    let listen = args.get(2).unwrap_or(&"127.0.0.1:3000".to_string()).to_string();

    let mut router = Router::new();
    router.post("/receive", move |req: &mut Request| -> IronResult<Response> {
        parse_hook(&config_dir[..], req)
    });

    let mut chain = Chain::new(router);
    chain.link_before(Read::<bodyparser::MaxBodyLength>::one(MAX_BODY_LENGTH));

    info!("Will listen on {}...", listen);
    Iron::new(chain).http(&listen[..]).unwrap();
}
