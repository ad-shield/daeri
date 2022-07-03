use std::net::SocketAddr;

use http::Uri;
use tracing_test::traced_test;

use daeri::mock;
use daeri::proxy::build_proxy;

#[tokio::test]
#[traced_test]
async fn test_simple() {
    tokio::spawn(mock::server());

    let app = build_proxy().unwrap();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    tokio::spawn(axum::Server::bind(&addr).serve(app.into_make_service()));

    let client = hyper::Client::new();
    client
        .get(Uri::from_static("http://localhost:3000"))
        .await
        .unwrap();
}
