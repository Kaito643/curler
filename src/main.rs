use clap::{App, Arg};
use reqwest::Error;
#[allow(unused_imports)]
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let matches = App::new("Curler")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Fetches a URL and displays the response")
        .arg(
            Arg::new("url")
                .help("The URL to fetch")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("headers")
                .short('I')
                .help("Display only the response headers"),
        )
        .get_matches();

    let url = matches.value_of("url").unwrap();
    let display_headers_only = matches.is_present("headers");

    match reqwest::get(url).await {
        Ok(response) => {
            if response.status().is_success() {
                if display_headers_only {
                    for (key, value) in response.headers() {
                        println!("{}: {:?}", key, value);
                    }
                } else {
                    match response.text().await {
                        Ok(body) => println!("{}", body),
                        Err(err) => eprintln!("Failed to read response body: {}", err),
                    }
                }
            } else {
                eprintln!("Request failed with status: {}", response.status());
            }
        }
        Err(err) => eprintln!("Failed to perform request: {}", err),
    }

    Ok(())
}
