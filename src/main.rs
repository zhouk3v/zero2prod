use std::net::TcpListener;
use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Bubble up the io::Error if we failed to bind the address
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to 8080");
    // Otherwise call .await on our Server
    run(listener)?.await
}
