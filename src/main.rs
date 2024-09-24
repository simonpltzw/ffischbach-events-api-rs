use ntex::web;

mod error;
mod services;
mod models;

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