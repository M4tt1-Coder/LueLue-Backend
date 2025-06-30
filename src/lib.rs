// crates inclusion
pub mod enums;
pub mod errors;
pub mod handlers;
pub mod middleware;
pub mod router;
pub mod sse;
pub mod types;

// Include the necessary dependencies
use tower_service::Service;
use worker::*;

use crate::router::router_provider;

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();
    Ok(router_provider::router().call(req).await?)
}

pub async fn root() -> &'static str {
    "Buddne!"
}

// Documentation
// https://github.com/cloudflare/workers-rs

// prod URL
// -> https://lue-lue-backend.geimat75.workers.dev/

// event emitter
// send server-sent events from the backend to the frontend
//
// nextjs -> EventSource API
// // https://developers.cloudflare.com/workers/runtime-apis/events/#server-sent-events

// necessary endpoints
//

// git feature branches _______
// utils -> implement util functions
// endpoints -> implement endpoints
// refactor -> refactor code
// => all are merge into dev
