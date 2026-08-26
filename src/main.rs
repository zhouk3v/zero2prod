use env_logger::Env;
use sqlx::PgPool;
use std::net::TcpListener;

use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Default to INFO logs and above if RUST_LOG is not set
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    // Bubble up the io::Error if we failed to bind the address
    let listener = TcpListener::bind(address)?;
    // Otherwise call .await on our Server
    run(listener, connection_pool)?.await
}
