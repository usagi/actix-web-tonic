// Note: This example is referenced from the official docs
// https://actix.rs/docs/server/

use actix_web::{
 get,
 http::StatusCode,
 HttpRequest,
 HttpResponse,
 Responder
};

#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder { "Welcome!" }

pub async fn teapot(_req: HttpRequest) -> impl Responder { HttpResponse::new(StatusCode::IM_A_TEAPOT).set_body("I like a green tea.🍵") }
