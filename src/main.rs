use warp::{Filter, Reply};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use serde_yaml;

#[derive(Deserialize, Serialize, Debug)]
struct MyData {
    field1: String,
    field2: i32,
}

#[derive(Deserialize, Debug)]
struct Config {
    ip_address: String,
    port: String,
    token: String,
}

#[tokio::main]
async fn main() {
    // Define the path to the configuration file
    let config_path = "local_data/config_data.yaml";
	
    // Read the configuration from the YAML file
    let config_str = std::fs::read_to_string(config_path)
        .expect("Failed to read config_tmp.yaml");
    let config: Config = serde_yaml::from_str(&config_str)
        .expect("Failed to parse YAML");
	
    // Parse the IP address and port from the config
    let ip_address_str = &config.ip_address;
    let port_str = &config.port;
	let token_str = &config.token;

    // Combine IP address and port into a single string
    let ip_port_str = format!("{}:{}", ip_address_str, port_str);

    // Parse the combined string into a SocketAddr
    let ip_address: SocketAddr = ip_port_str.parse().unwrap();
	
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