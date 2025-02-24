use reqwest::Error;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Collect the command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the user provided a URL
    if args.len() < 2 {
        eprintln!("Usage: curler <URL>");
        std::process::exit(1);
    }

    // Get the URL from the command-line arguments
    let url = &args[1];

    // Perform the HTTP GET request
    match reqwest::get(url).await {
        Ok(response) => {
            // Check if the response status is success
            if response.status().is_success() {
                // Print the response body
                match response.text().await {
                    Ok(body) => println!("{}", body),
                    Err(err) => eprintln!("Failed to read response body: {}", err),
                }
            } else {
                eprintln!("Request failed with status: {}", response.status());
            }
        }
        Err(err) => eprintln!("Failed to perform request: {}", err),
    }

    Ok(())
}
