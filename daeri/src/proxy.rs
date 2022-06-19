use axum::{error_handling::HandleError, middleware, Router};
use hyper::{Client, StatusCode, Uri};
use tokio::task;
use tower::ServiceBuilder;
use tower_http::gateway::Gateway;

use crate::handler::print_request_response;
use crate::Error;

pub fn build_proxy() -> Result<Router, Error> {
    let client = Client::new();
    let gateway = Gateway::new(client, Uri::from_static("http://127.0.0.1:3000"))?;

    let app = Router::new()
        .nest(
            "/",
            HandleError::new(gateway, |_| async { StatusCode::BAD_GATEWAY }),
        )
        .layer(
            ServiceBuilder::new()
                .layer(middleware::from_fn(print_request_response))
                .and_then(|body| async {
                    eprintln!("hello from and_then");
                    task::spawn_blocking(move || {
                        std::thread::sleep(std::time::Duration::from_secs(1));
                    })
                    .await
                    .unwrap();
                    eprintln!("waited long enough");
                    Ok(body)
                }),
        );

    Ok(app)
}
