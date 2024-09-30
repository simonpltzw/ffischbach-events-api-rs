use std::env;

use diesel::{Connection, PgConnection};
use dotenvy::dotenv;
use ntex::web;

mod error;
mod services;
mod models;
mod schema;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| {
        web::App::new()
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