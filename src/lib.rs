use reqwest::{blocking::multipart, blocking::Client, Result};
use serde::{Deserialize, Serialize};
use std::env;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadResponse {
    pub shortcode: String,
    pub status: i8,
}

pub fn upload(filepath: &str) -> Result<UploadResponse> {
    let home_path = dirs::home_dir().map(|a| a.join(".streamable")).unwrap();

    dotenv::from_path(home_path.as_path()).unwrap();

    let username = env::var("STREAMABLE_USERNAME").unwrap();
    let password = env::var("STREAMABLE_PASSWORD").unwrap();

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

    let response_as_json = resp.json::<UploadResponse>().unwrap();

    Ok(response_as_json)
}

pub fn get_username_password() -> (String, String) {
    println!("Please input Your username and password");
    let username = rpassword::read_password_from_tty(Some("Username: ")).unwrap();
    let password = rpassword::read_password_from_tty(Some("Password: ")).unwrap();
    (username, password)
}

pub fn setup() -> Result<()> {
    let env = get_username_password();

    let text = format!(
        "STREAMABLE_USERNAME={}\nSTREAMABLE_PASSWORD={}",
        env.0, env.1
    );

    let home_path = dirs::home_dir().unwrap();
    let config_path = home_path.join(".streamable");
    std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(config_path)
        .expect("can not create config file")
        .write_all(text.as_bytes())
        .unwrap();

    println!(
        "saved your login params in {} as .streamable",
        home_path.to_str().unwrap()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::upload;

    #[test]
    fn streamable_upload() {
        let _ = upload("./media/sample.mp4").unwrap();
    }
}
