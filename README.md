# Starling Webhooks

Rust types for handling data from Starling Bank's webhook API.

## Usage

The following example shows a basic HTTP server built with `hyper` and `tokio`
that deserializes payload data at `/feed-item` into a `WebhookFeedItem` type
and displays it in the debug format. All other routes return 404s.

The source for this example can be found in `/examples/hyper-server` with the
dependencies and versions.

```rust
use std::convert::Infallible;
use std::net::{Ipv4Addr, SocketAddrV4};

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use starling_webhooks::WebhookFeedItem;

async fn router(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let response = match req.uri().path() {
        "/feed-item" => handle_feed_item(req).await,
        _ => Response::builder().status(404).body(Body::empty()).unwrap(),
    };

    Ok(response)
}

async fn handle_feed_item(mut req: Request<Body>) -> Response<Body> {
    // Get the body itself
    let body = hyper::body::to_bytes(req.body_mut())
        .await
        .expect("Failed to get the body");

    // Deserialize it and display it
    let webhook: WebhookFeedItem =
        serde_json::from_slice(&body).expect("Failed to deserialize the content");

    dbg!(&webhook);

    Response::default()
}

#[tokio::main]
async fn main() {
    let service = make_service_fn(|_| async { Ok::<_, Infallible>(service_fn(router)) });

    let addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 4000).into();
    let server = Server::bind(&addr).serve(service);

    server.await.expect("Failed to handle errors");
}
```
