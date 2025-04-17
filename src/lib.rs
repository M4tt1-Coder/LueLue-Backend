use axum::{routing::get, Router};
use tower_service::Service;
use worker::*;

fn router() -> Router {
    Router::new().route("/", get(root))
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();
    Ok(router().call(req).await?)
}

pub async fn root() -> &'static str {
    "Buddne!"
}

// Documentation
// https://github.com/cloudflare/workers-rs

// prod URL
// -> https://lue-lue-backend.geimat75.workers.dev/

// needed endpoints
// 

// git feature branches _______
// utils -> implement util functions
// endpoints -> implement endpoints
// refactor -> refactor code
// => all are merge into dev

