use axum::body::{Bytes, HttpBody};
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::{error_handling::HandleError, middleware, Router};
use http::Request;
use hyper::{Body, Client, StatusCode, Uri};
use tokio::task;
use tower::ServiceBuilder;
use tower_http::gateway::Gateway;

use crate::Error;

pub fn build_proxy() -> Result<Router, Error> {
    let client = Client::new();
    let gateway = Gateway::new(client, Uri::from_static("http://127.0.0.1:4000"))?;

    let app = Router::new()
        .nest(
            "/",
            HandleError::new(gateway, |_| async { StatusCode::BAD_GATEWAY }),
        )
        .layer(
            ServiceBuilder::new()
                .layer(middleware::from_fn(modify))
                .and_then(|body| async {
                    eprintln!("hypothetically modifying result");
                    task::spawn_blocking(move || {
                        std::thread::sleep(std::time::Duration::from_secs(1));
                    })
                    .await
                    .unwrap();
                    eprintln!("protection has finished");
                    Ok(body)
                }),
        );

    Ok(app)
}

enum ModifyStrategy {
    Identity,
    Fast,
    Slow,
}

fn dispatch(res: &impl IntoResponse) -> ModifyStrategy {
    // TODO
    ModifyStrategy::Identity
}

fn modify_fast(data: Bytes) -> Bytes {
    todo!()
}

fn modify_slow(data: Bytes) -> Bytes {
    todo!()
}

async fn modify(
    req: Request<Body>,
    next: Next<Body>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let res = next.run(req).await;

    let modified = match dispatch(&res) {
        ModifyStrategy::Identity => res.into_body(),
        ModifyStrategy::Fast => res.map_data(modify_fast).boxed_unsync(),
        ModifyStrategy::Slow => res.map_data(modify_slow).boxed_unsync(),
    };

    Ok(modified)
}
