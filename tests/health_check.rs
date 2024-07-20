use zero2prod::configuration::{get_configuration, DatabaseSettings};
use zero2prod::startup::run;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool
}

/// Spin up an instance of our application
/// and returns its address (i.e. http://127.0.0.1:XXXX)
async fn spawn_app() -> TestApp {
    // Bind to port 0 to get the OS to scan for an available port for testing
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // Retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    // Spin up a postgres connection pool
    let mut configuration = get_configuration().expect("Failed to read configuration");
    configuration.database.database_name = Uuid::new_v4().to_string();
    let connection_pool = configure_database(&configuration.database).await;

    // Create the server
    let server = run(listener, connection_pool.clone()).expect("Failed to bind address");
    // Launch the server as a background task with tokio::spawn
    // tokio::spawn takes a future and hands it over to the runtime for polling, without waiting for its completion
    // tokio::spawn will return a handle, but we will not need it for this function
    let _ = tokio::spawn(server);
    // Return an instance of TestApp
    TestApp {
        address,
        db_pool: connection_pool
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create database
    let mut connection = PgConnection::connect(
        &config.connection_string_without_db()
    ).await.expect("Failed to connect to Postgres");
    connection.execute(format! (r#"CREATE DATABASE "{}";"#, config.database_name).as_str()).await.expect("Failed to create database.");

    // Migrate database
    let connection_pool = PgPool::connect(&config.connection_string()).await.expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq! (saved.email, "ursula_le_guin@gmail.com");
    assert_eq! (saved.name, "le guin");

}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
