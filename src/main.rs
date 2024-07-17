use std::net::TcpListener;

use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Create a TcpListener to bind to port 8000
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind to 8000");
    // Bubble up the io::Error if we fail to bind the address
    // Otherwise call .await on the Server
    run(listener)?.await
}
