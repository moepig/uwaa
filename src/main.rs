use serde::{Deserialize, Serialize};
 
#[derive(Serialize, Deserialize, Debug)]
pub struct UserTokenPostRequest {
    token: String,
    channel: String,
    text: String,
}

fn main() {
    println!("Hello, world!");
}
