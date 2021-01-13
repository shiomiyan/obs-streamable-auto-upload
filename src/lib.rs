use reqwest::{blocking::multipart, blocking::Client, blocking::Response, Result};
use std::env;
use std::io::prelude::*;

pub fn upload(filepath: &str) -> Result<Response> {
    let home_path = dirs::home_dir()
        .and_then(|a| Some(a.join(".streamable")))
        .unwrap();

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

    // dbg!(&resp);

    Ok(resp)
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

    println!("saved your login params in {}", home_path.to_str().unwrap());
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::upload;

    #[test]
    fn streamable_upload() {
        let result = upload("./media/sample.mp4");
        dbg!(&result);

        match result {
            Ok(v) => println!("{:?}", v),
            Err(e) => panic!("Error: {}", e),
        }
    }
}
