use axum::{extract::Path, routing::get, Router};
use std::net::SocketAddr;

pub async fn server() {
    let app = Router::new().route(
        "/*path",
        get(|Path(path): Path<String>| async move { format!("Hello from `Get {path}`") })
            .post(|Path(path): Path<String>| async move { format!("Hello from `POST {path}`") }),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    println!("server listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
