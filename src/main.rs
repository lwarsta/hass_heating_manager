use warp::{Filter, Reply};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct MyData {
    field1: String,
    field2: i32,
}

#[tokio::main]
async fn main() {
    // Define a filter for the specific path and POST method
    let my_route = warp::path!("my_route" / "post")
        .and(warp::post())
        .and(warp::body::json()) // Automatically deserialize JSON data
        .map(|data: MyData| {
            // Handle the received data
            warp::reply::json(&data) // Echo the received data as JSON
        });

    // Combine filters and start the warp server
    warp::serve(my_route).run(([127, 0, 0, 1], 3030)).await;
}