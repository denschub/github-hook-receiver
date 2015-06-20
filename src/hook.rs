extern crate rustc_serialize;

#[derive(Debug)]
pub struct GithubHook {
    pub signature: String,
    pub payload: rustc_serialize::json::Json
}

pub fn receive(hook: GithubHook) {
    println!("{:?}", hook);
}
