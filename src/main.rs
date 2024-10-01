use std::env;

use diesel::{Connection, PgConnection};
use diesel::r2d2::{self, ConnectionManager};
use dotenvy::dotenv;
use ntex::web;

mod error;
mod services;
mod models;
mod schema;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // set up database connection pool
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    web::HttpServer::new(move || {
        web::App::new()
            // Add connetion pool
            .state(pool.clone())
            // Register event endpoints
            .configure(services::event::ntex_config)
            // Register swagger endpoints
            .configure(services::openapi::ntex_config)
            // Default endpoint for unregisterd endpoints
            .default_service(web::route().to(services::default))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}