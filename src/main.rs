use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Initialize the subscriber
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    // Get configuration settings
    let configuration = get_configuration().expect("Faild to read configuration.");
    // Create database connection pool
    let connection_pool =
        PgPool::connect_lazy(&configuration.database.connection_string().expose_secret())
            .expect("Failed to create Postgres connection pool.");
    let address = format!("{}:{}", configuration.application.host ,configuration.application.port);
    // Create a TcpListener to bind to port in settings
    let listener = TcpListener::bind(address)?;
    // Bubble up the io::Error if we fail to bind the address
    // Otherwise call .await on the Server
    run(listener, connection_pool)?.await
}
