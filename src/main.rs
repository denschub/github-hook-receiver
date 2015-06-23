extern crate bodyparser;
extern crate iron;
extern crate persistent;
extern crate router;

#[macro_use] extern crate hyper;

mod hook;

use std::env;
use std::thread;

use hook::*;
use iron::prelude::*;
use iron::status;
use persistent::Read;
use router::{Router};

header! {
    (XHubSignature, "X-Hub-Signature") => [String]
}

const MAX_BODY_LENGTH: usize = 1024 * 1024 * 10;

fn parse_hook(req: &mut Request) -> IronResult<Response> {
    let body = req.get::<bodyparser::Raw>();
    let signature = req.headers.get::<XHubSignature>()
        .unwrap_or(&XHubSignature("".to_string()))
        .to_string();

    match body {
        Ok(Some(body)) => {
            thread::spawn(move || {
                let hook = GithubHook {
                    payload: body,
                    signature: signature,
                };
                hook::receive(hook);
            });
            Ok(Response::with(status::Ok))
        },
        _ => Ok(Response::with(status::NotFound))
    }
}

fn main() {
    let listen = env::var("LISTEN").unwrap_or("127.0.0.1:3000".to_string());
    println!("Will listen on {}...", listen);

    let mut router = Router::new();
    router.post("/receive", parse_hook);

    let mut chain = Chain::new(router);
    chain.link_before(Read::<bodyparser::MaxBodyLength>::one(MAX_BODY_LENGTH));

    Iron::new(chain).http(&*listen).unwrap();
}
