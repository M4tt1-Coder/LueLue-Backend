// crates inclusion
pub mod enums;
pub mod errors;
pub mod handlers;
pub mod logic;
pub mod middleware;
pub mod router;
pub mod status;
pub mod types;

// Include the necessary dependencies
use log::warn;
use tower_service::Service;
use worker::*;

use crate::router::router_provider;

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    // TODO: Set up database repositories for all types relevant for direct data exchange

    // Get the database binding -> access to D1 database
    let _database = env.d1("DB").map_err(|err| {
        warn!("{err}");
        worker::Error::RustError("DB binding not found".to_string())
    })?;
    console_error_panic_hook::set_once();
    Ok(router_provider::router().call(req).await?)
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
