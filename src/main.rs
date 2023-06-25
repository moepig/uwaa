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

fn main() {
    println!("Hello, world!");
}
