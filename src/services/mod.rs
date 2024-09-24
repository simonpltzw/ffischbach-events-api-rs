use ntex::web;

pub mod openapi;
pub mod event;

pub async fn default() -> web::HttpResponse {
  web::HttpResponse::NotFound().finish()
}
