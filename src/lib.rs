use dotenv::dotenv;
use reqwest::{blocking::multipart, blocking::Client, Result};
use std::env;


pub fn upload(filepath: &str) -> Result<()> {
    dotenv().ok();
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

    Ok(())
}


#[cfg(test)]
mod tests {
    use crate::upload;

    #[test]
    fn streamable_upload() {
        let result = upload("./media/sample.mp4");
        match result {
            Ok(v) => println!("{:?}", v),
            Err(e) => panic!("Error: {}", e)
        }
    }
}
