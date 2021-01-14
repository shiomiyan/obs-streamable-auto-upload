use clap::{App, Arg};
use uploader::{setup, upload};

fn main() {
    let matches = App::new("streamable-upload")
        .version("0.1")
        .about("upload your video to streamable")
        .author("shiomiya")
        .subcommand(
            App::new("upload").about("input path to your video").arg(
                Arg::new("PATH")
                    .about("input video path")
                    .value_name("PATH")
                    .takes_value(true),
            ),
        )
        .subcommand(App::new("setup").about("set your streamable.com username and password"))
        .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("upload") {
        let path = matches.value_of("PATH").unwrap();
        let response = upload(path).unwrap();
        match response.status {
            1 => {
                let shortlink = format!("https://www.streamable.com/{}", response.shortcode);
                println!("Upload success! check {}", shortlink);
                std::process::exit(0)
            }
            _ => {
                println!("Failed to upload.");
                std::process::exit(1)
            }
        }
    }

    if let Some(ref _matches) = matches.subcommand_matches("setup") {
        let result = setup();
        match result {
            Ok(v) => {
                println!("{:?}", v);
                std::process::exit(0)
            }
            Err(e) => panic!("Error: {}", e),
        }
    }
}
