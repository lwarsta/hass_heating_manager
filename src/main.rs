use std::process::Command;
use warp::{Filter, Reply};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use serde_json;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, AUTHORIZATION};
use serde_json::json;
use std::fs;
use serde::ser::StdError;

// Data structure for messaging between Home Assistant UI.
#[derive(Deserialize, Serialize, Debug)]
struct DataHass {
	entity_cat: i32,
	entity_id: String,
	data_type: i32,
	data_unit: String,
	data_str: String,
	data_int: i32,
	data_float: f32,
	data_bool: bool,
	date_time: String,
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

async fn make_post_request(url: &str, data: &str, token: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Construct the request headers
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("{}", token)).unwrap(),
    );

    // Construct the payload as a JSON object
    let payload = json!({
        "title": "REST Call Received",
        "message": format!("data: {}", data),
    });
	
    // Send the POST request
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .headers(headers)
        .json(&payload) // Use the correct json! macro
        .send()
        .await?;

    // Check the response status
    if let Err(err) = response.error_for_status() {
        eprintln!("Error making POST request: {:?}", err);
        return Err(Box::new(err));
    }

    Ok(())
}

// Function to execute a Julia script.
fn execute_julia_script() {
    let output = Command::new("julia") // "/usr/local/julia/bin/julia"
        .arg("/app/hello_world.jl") // Path to Julia script
        .output()
        .expect("Failed to execute Julia script.");

    println!("Julia output: {}", String::from_utf8_lossy(&output.stdout));
}

#[tokio::main]
async fn main() {
    // Uncomment and use the following line for checking directory contents
    print_directory_contents("/app");
	print_directory_contents("/usr/local/julia");
	print_directory_contents("/usr/local/bin");
	
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
    let my_route = warp::path!("from_hass" / "post")
        .and(warp::post())
        .and(warp::body::json()) // Automatically deserialize JSON data
        .map(|data: DataHass| {
            // Print the received data to the console
            println!("Received data: {:?}", data);
            // Handle the received data
            warp::reply::json(&data) // Echo the received data as JSON
        });
	
    // Print a message indicating that the server is starting
    println!("Server started at {}", ip_address);

    // Execute the Julia script
    execute_julia_script();

    // Make a test POST call to the Home Assistant User Interface.
    println!("Make test POST call to the Home Assistant User Interface:");
    let url_hass = "http://homeassistant.local:8123/api/services/persistent_notification/create";
    let data_hass = "{
		\"entity_cat\": 1, 
		\"entity_id\": \"0001\", 
		\"data_type\": 2, 
		\"data_unit\": \"degree\", 
		\"data_str\": \"\", 
		\"data_int\": 0, 
		\"data_float\": 10.0, 
		\"data_bool\": false,
		\"date_time\": \"2023-01-01T00:00:00Z\"
		}";
    if let Err(err) = make_post_request(url_hass, data_hass, hass_token).await {
        eprintln!("Error making POST request: {:?}", err);
    }

    // Combine filters and start the warp server
    warp::serve(my_route).run(ip_address).await;
}