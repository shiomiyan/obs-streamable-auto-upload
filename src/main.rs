use clap::{App, Arg};
use reqwest::StatusCode;

use uploader::upload;

fn main() {
    let matches = App::new("streamable-upload")
        .version("1.0")
        .about("upload your video to streamable")
        .author("shiomiya")
        .arg(
            Arg::new("INPUT")
                .about("input path to your video")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    if let Some(path) = matches.value_of("INPUT") {
        let response = upload(path).unwrap();
        match response.status() {
            StatusCode::OK => println!("Upload sccess!"),
            s => println!("Received response status: {:?}", s)
        }
    }
}
