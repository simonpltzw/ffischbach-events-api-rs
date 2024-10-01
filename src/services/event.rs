use diesel::prelude::*;
use ntex::web::{self};
use serde::Deserialize;
use utoipa::IntoParams;

use crate::models::event::EventOutput;

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
struct GetEventParams {
  /// Limits the number of items returned from a collection. Defaults to 100.
  top: Option<u32>,
  /// Specifies the number of items to skip in a collection before returning the remaining items.
  skip: Option<u32>
}

/// List all events
#[utoipa::path(
  get,
  path = "/events",
  params(GetEventParams),
  responses(
    (status = 200, description = "List of Events", body = [EventOutput]),
  ),
)]
#[web::get("/events")]
pub async fn get_events(pool: web::types::State<crate::DbPool>, query: web::types::Query<GetEventParams>) -> web::HttpResponse {
  let top = query.top.unwrap_or(100);
  let skip = query.skip.unwrap_or_default();

  use crate::schema::events::dsl::*;

  let mut conn = pool.get().expect("couldn't get db connection from pool");

  let results = events
        .limit(top as i64)
        .offset(skip as i64)
        .select(EventOutput::as_select())
        .load(&mut conn)
        .expect("Error loading posts");

  web::HttpResponse::Ok().json(&results)
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