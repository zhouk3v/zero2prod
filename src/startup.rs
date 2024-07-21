use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

// Return an instance of a HttpServer using a TcpListener
pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    // Wrap database connection pool in a smart pointer (Equivelent to Arc)
    let db_pool = Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            // apply the Logger middleware
            // Note that this will emit logs, but does not process them (read: print them), see main.rs for the logger setup
            .wrap(TracingLogger::default())
            // routes for the endpoints
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            // App state (db connections, etc)
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
