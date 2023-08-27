use warp::Filter;

#[tokio::main]
async fn main() {
    // Define a filter that responds to GET requests at the "/hello" path
    let hello = warp::path!("hello" / "world")
        .map(|| warp::reply::html("Hello, world!"));

    // Combine the filters and start the warp server
    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
