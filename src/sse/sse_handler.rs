use axum::response::sse::{Event, Sse};

use axum::response::sse::KeepAlive;

/// Handler for Server-Sent Events (SSE)
///
/// This handler streams events to the client, sending an event every second
///
/// # Example Usage
/// ```rust
/// use axum::Router;
/// use axum::routing::get;
/// let app = Router::new().route("/sse", get(sse_handler::sse_handler));
/// ```
///
/// # Returns a Router with the SSE handler
pub fn sse_handler() -> Sse<impl futures::Stream<Item = Result<Event, axum::Error>>> {
    let stream = async_stream::stream! {
        // TODO: Implement your event generation logic here
        loop {
            yield Ok(Event::default().data(format!("That's a update SSE at {}", chrono::Utc::now()
            )));
            tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
        }
    };

    Sse::new(stream).keep_alive(KeepAlive::default())
}
