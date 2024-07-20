use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use sqlx::PgPool;
use std::net::TcpListener;


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Get configuration settings
    let configuration = get_configuration().expect("Faild to read configuration.");
    // Create database connection pool
    let connection_pool = PgPool::connect(
        &configuration.database.connection_string()
    ).await.expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    // Create a TcpListener to bind to port in settings
    let listener = TcpListener::bind(address)?;
    // Bubble up the io::Error if we fail to bind the address
    // Otherwise call .await on the Server
    run(listener, connection_pool)?.await
}
