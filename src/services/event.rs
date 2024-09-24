use ntex::web;

/// List all events
#[utoipa::path(
  get,
  path = "/events",
  responses(
    (status = 200, description = "List of Events", body = [EventOutput]),
  ),
)]
#[web::get("/events")]
pub async fn get_events() -> web::HttpResponse {
  web::HttpResponse::Ok().finish()
}

/// Get an event by id
#[utoipa::path(
  get,
  path = "/events/{id}",
  params(
    ("id", description = "Event id")
  ),
  responses(
    (status = 200, description = "Event found", body = EventDetailOutput),
    (status = 404, description = "Event not found", body = HttpError),
  ),
)]
#[web::get("/events/{id}")]
pub async fn get_event(path: web::types::Path<(u32,)>) -> web::HttpResponse {
  web::HttpResponse::Ok().content_type("application/json").body(format!("{}", path.0))
}

pub fn ntex_config(cfg: &mut web::ServiceConfig) {
  cfg.service(get_events);
  cfg.service(get_event);
}