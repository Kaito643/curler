# Minimal Rust Curl Demo

Minimal http request tool written with Rust Programming Language.

## Features
- Sends a GET request to a IP Address or URL Address
- Can detect & show response packet headers

## Prerequisites
- Rust & Cargo
- macOS (tested on macOS systems) - currently working on implementing this to linux

## Installation
### From Source
git clone https://github.com/Kaito643/curler.git 

cd curler

### Install Local User Wide 
This part is optional but if you do not do this you need to locate curler directory everytime

cargo build --release

sudo cp target/release/curler /usr/local/bin/

## Usage
cargo run -- <TARGET_ADDRESS> 
### Example:
cargo run -- google.com

cargo run -- -I google.com

### If installed in environment
curler <TARGET_ADDRESS>
curler -I <TARGET_ADDRESS>

# Example:
curler google.com
curler -I google.com

## Limitations 
This was a demo for me to get in http packets with rust libraries. I implemented -I parameter from original curl tool but maybe I will try to implement other arguments and features in the future.
- Output is not pretty
- Not many features only -I works.

## Technical Details & How it works
- We send a get request packet with the help of reqwest library.
- If argument given, response packet's headers are shown. If else body is shown in the output.
- If response packet is not counted as "succesfull" to library it outputs error.

I will explain the initial commit in this blog writing if anyone is further interested

[Pinger-How-It-Works](https://umut-yildiz-blogs.notion.site/Curler-How-it-works-1a82e945179b8003a364d653d81bcf35?pvs=4)

# Troubleshooting
## Possible errors are;
### 1. "Failed to perform request: [error]"
Caused by;
- Network connectivity issues
- Invalid URL format
- DNS resolution failure
- Server not responding
- TLS/SSL configuration issues

Simple Solutions;
- Verify the URL format includes http:// or https://
- Test network connectivity using ping <domain> or curl <url>
- Check DNS resolution with nslookup <domain> or dig <domain>

### 3. "Request failed with status: [HTTP status]"
Caused by;
- Server-side errors (5xx status codes)
- Client-side errors (4xx status codes)
- Authentication requirements
- Resource not found

Simple Solutions;
- Check the meaning of the HTTP status code (e.g., 404 = Not Found)
- Verify the full URL path and query parameters
- Check for authentication requirements (API keys, tokens, etc.)

### 4. "Failed to read response body: [error]"
Caused by;
- Non-text response (binary data)
- Invalid UTF-8 encoding
- Large response payload
- Connection interrupted during transfer

Simple Solutions;
- Check Content-Type header using the -I flag to verify if it's text-based
- Try reading as binary data instead (response.bytes().await)
- Test with different endpoints that return simple text responses

## Conclusion
Please contact me if I could be of any help or if you have suggestions to improve. I will be developing this project in future.
