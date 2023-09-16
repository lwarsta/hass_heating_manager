use warp::{Filter, Reply};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use serde_yaml;
use serde_json;
use std::fs;

// Incoming data from Home Assistant UI.
#[derive(Deserialize, Serialize, Debug)]
struct MyData {
    field1: String,
    field2: i32,
}

// Configuration options saved into a json file in the addon data directory.
#[derive(Deserialize, Debug)]
struct Options {
	floor_area: i32,
	stories: i32,
	insulation_u_value: f32,
    listen_ip: String,
    port: String,
    hass_token: String,
}

// Function to read and print the contents of a directory in the server.
fn print_directory_contents(dir_path: &str) {
    match fs::read_dir(dir_path) {
        Ok(entries) => {
            println!("Contents of directory '{}':", dir_path);
            for entry in entries {
                match entry {
                    Ok(dir_entry) => {
                        // Get the file name as a string
                        if let Some(file_name) = dir_entry.file_name().to_str() {
                            // Check if it's a file or a directory
                            if dir_entry.file_type().map_or(false, |ft| ft.is_dir()) {
                                println!("Directory: {}", file_name);
                            } else {
                                println!("File: {}", file_name);
                            }
                        }
                    }
                    Err(err) => eprintln!("Error reading directory entry: {}", err),
                }
            }
        }
        Err(err) => eprintln!("Error reading directory: {}", err),
    }
}

#[tokio::main]
async fn main() {
    // Uncomment and use the following line for checking directory contents.
    // print_directory_contents("/data");
	
    // Define the path to the options.json file
    let options_path = "/data/options.json";

    // Read the options.json file as a string
    let options_str = match fs::read_to_string(options_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading options.json: {}", err);
            return;
        }
    };

    // Parse the options JSON string into an Options struct
    let options: Options = match serde_json::from_str(&options_str) {
        Ok(parsed_options) => parsed_options,
        Err(err) => {
            eprintln!("Error parsing options.json: {}", err);
            return;
        }
    };
	
    // Extract option data from the options.json file.
	let floor_area = &options.floor_area;
	let stories = &options.stories;
	let insulation_u_value = &options.insulation_u_value;
    let listen_ip = &options.listen_ip;
    let port = &options.port;
	let hass_token = &options.hass_token;
	
	// Partially mask the hass token for printing.
	let masked_token = if options.hass_token.len() > 4 {
		let last_part = &options.hass_token[options.hass_token.len() - 4..];
		let masked_part = "*".repeat(options.hass_token.len() - 4);
		format!("{}{}", masked_part, last_part)
	} else {
		// If the token is too short, just print it as is
		options.hass_token.clone()
	};
	
    // Print the individual option variables
    println!("floor_area: {}", options.floor_area);
    println!("stories: {}", options.stories);
    println!("insulation_u_value: {}", options.insulation_u_value);
    println!("listen_ip: {}", options.listen_ip);
    println!("port: {}", options.port);
    println!("hass_token: {}", masked_token);

    // Combine IP address and port into a single string
    let ip_port = format!("{}:{}", listen_ip, port);

    // Parse the combined string into a SocketAddr
    let ip_address: SocketAddr = ip_port.parse().unwrap();
	
    // Define a filter for the specific path and POST method
    let my_route = warp::path!("my_route" / "post")
        .and(warp::post())
        .and(warp::body::json()) // Automatically deserialize JSON data
        .map(|data: MyData| {
            // Print the received data to the console
            println!("Received data: {:?}", data);
            // Handle the received data
            warp::reply::json(&data) // Echo the received data as JSON
        });
	
    // Print a message indicating that the server is starting
    println!("Server started at {}", ip_address);

    // Combine filters and start the warp server
	warp::serve(my_route).run(ip_address).await;
}