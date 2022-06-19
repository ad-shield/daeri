use axum::{
    extract::Path,
    middleware::{self},
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tokio::task;
use tower::ServiceBuilder;

use crate::handler::print_request_response;

pub async fn server() {
    let app = Router::new()
        .route(
            "/*path",
            get(|Path(path): Path<String>| async move { format!("Hello from `Get {path}`") }).post(
                |Path(path): Path<String>| async move { format!("Hello from `POST {path}`") },
            ),
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

    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    println!("server listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
