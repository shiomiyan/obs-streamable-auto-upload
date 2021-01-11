use dotenv::dotenv;
use reqwest::{blocking::multipart, blocking::Client, Result};
use std::env;

fn main() {
    dotenv().ok();
    let username = env::var("STREAMABLE_USERNAME").unwrap();
    let password = env::var("STREAMABLE_PASSWORD").unwrap();

    let _ = upload(&username, &password, "./media/sample.mp4");
}

pub fn upload(username: &str, password: &str, filepath: &str) -> Result<()> {
    let endpoint = "https://api.streamable.com/upload";

    let form = multipart::Form::new()
        .file("file", std::path::Path::new(filepath))
        .unwrap_or_else(|e| panic!("Error: {}", e));

    let client = Client::new();
    let resp = client
        .post(endpoint)
        .basic_auth(username, Some(password))
        .multipart(form)
        .send()
        .unwrap_or_else(|e| panic!("Error: {}", e));

    // dbg!(&resp);

    Ok(())
}
