use std::env;
use std::process;
use serde::{Deserialize, Serialize};
 
// https://api.slack.com/methods/chat.postMessage#args
#[derive(Serialize, Deserialize, Debug)]
pub struct UserTokenPostRequest {
    token: String,
    channel: String,
    text: String,
}

// https://api.slack.com/methods/chat.postMessage#examples
#[derive(Serialize, Deserialize, Debug)]
pub struct UserTokenPostResponse {
    ok: bool,
}

fn must_var(s: &str) -> String {
    let ret_val  = match env::var(s) {
        Ok(val) => val,
        Err(err) => {
            println!("{}: {}", err, s);
            process::exit(1);
        },
    };

    return ret_val;
}

fn main() {
    let token = must_var("UWAA_TOKEN");
    let channel= must_var("UWAA_CHANNEL");

    println!("Hello, world!");
}
