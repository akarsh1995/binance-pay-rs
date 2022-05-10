use std::net::SocketAddr;

use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Extension, Router};
use bpay::c2b::webhook::notification::Notification;
use bpay::c2b::webhook::verification::{Verifier, Verify};
use bpay::client::Client;
use reqwest::StatusCode;
use tower::ServiceBuilder;

#[tokio::main]
async fn main() -> bpay::errors::Result<()> {
    let client = Client::from_env();
    let signature_verifier = Verifier::from_api(&client).await?;
    create_server(signature_verifier).await;
    Ok(())
}

async fn create_server(verifier: Verifier) {
    let signature_verifier = std::sync::Arc::new(verifier);
    let app = Router::new()
        .route("/", post(notification_request_handler))
        .layer(ServiceBuilder::new().layer(Extension(signature_verifier)));

    // Make sure you have added tls layer of security to your proxy server.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let server = axum::Server::bind(&addr);
    server.serve(app.into_make_service()).await.unwrap();
}

async fn notification_request_handler(
    Extension(ref mut verifier): Extension<std::sync::Arc<Verifier>>,
    headers: HeaderMap,
    body: String,
) -> impl IntoResponse {
    if let Err(e) = verifier.verify(&headers, body.as_str()) {
        println!("Encountered error while verifying the signature: {e}");
        (StatusCode::BAD_REQUEST, "Undefined error".to_string())
    } else {
        if let Ok(notification) = Notification::try_from(body.as_str()) {
            match notification {
                Notification::Order {
                    biz_id: _,
                    biz_status: _,
                    order_detail: _,
                } => {
                    // Do something with the order notification
                }
                _ => {
                    // Do something with other variants
                }
            }
            (StatusCode::OK, "OK".to_string())
        } else {
            (
                StatusCode::BAD_REQUEST,
                "Could not parse the body.".to_string(),
            )
        }
    };
}
