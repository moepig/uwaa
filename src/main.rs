use std::env;
use std::process;
use std::error::Error;
use serde::{Deserialize, Serialize};
 
// https://api.slack.com/methods/chat.postMessage#args
#[derive(Serialize, Deserialize, Debug)]
pub struct UserTokenPostRequest {
    channel: String,
    text: String,
}

// https://api.slack.com/methods/chat.postMessage#examples
#[derive(Serialize, Deserialize, Debug)]
pub struct UserTokenPostResponse {
    ok: bool,
}

fn must_env(s: &str) -> String {
    let ret_val  = match env::var(s) {
        Ok(val) => val,
        Err(err) => {
            println!("{}: {}", err, s);
            process::exit(1);
        },
    };

    return ret_val;
}

fn optional_env(key: &str, default: &str) -> String {
    let ret_val  = match env::var(key) {
        Ok(val) => val,
        Err(_) => default.to_string(),
    };

    return ret_val;
}

fn main() -> Result<(), Box<dyn Error>> {
    let token = must_env("UWAA_TOKEN");
    let channel= must_env("UWAA_CHANNEL");
    let endpoint = optional_env("UWAA_ENDPOINT", "https://slack.com/api/chat.postMessage");

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("args[1] is required.");
        process::exit(1);
    }

    let client = reqwest::blocking::Client::new();

    let params = UserTokenPostRequest {
        channel: channel,
        text: args[1].clone(),
    };

    let response_body: UserTokenPostResponse = client.post(endpoint)
        .header(reqwest::header::AUTHORIZATION, format!("Bearer {}", token))
        .json(&params)
        .send()?
        .json()?;

    println!("{:?}", params);
    println!("{:?}", response_body);

    return Ok(())
}

#[test]
fn request() {
    let mut server = mockito::Server::new();

    let url = server.url();

    let mock = server.mock("POST", "/")
    .with_status(200)
    .with_header("content-type", "application/json")
    .with_body(r#"{"ok": true}"#)
    .create();

    env::set_var("UWAA_TOKEN", "test_token");
    env::set_var("UWAA_CHANNEL", "CTESTCHANNEL");
    env::set_var("UWAA_ENDPOINT", url);

    let result = main();
    assert!(result.is_ok());

    mock.assert();
    mock.remove();
}    
