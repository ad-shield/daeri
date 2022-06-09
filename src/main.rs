use axum::{
    body::{Body, Bytes},
    extract::{Extension, Path},
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{any, get},
    Router,
};
use std::net::SocketAddr;
use tokio::task;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use daeri::handler::proxy_handler;
use daeri::Client;
use tower::ServiceBuilder;

// struct ModificationLayer;

// impl<S> Layer<S> for ModificationLayer {
//     type Service = ModificationMiddleware<S>;

//     fn layer(&self, inner: S) -> Self::Service {
//         ModificationMiddleware { inner }
//     }
// }

// #[derive(Clone)]
// struct ModificationMiddleware<S> {
//     inner: S,
// }

// impl<S> Service<Request<Body>> for ModificationMiddleware<S>
// where
//     S: Service<Request<Body>, Response = Response> + Send + 'static,
//     S::Future: Send + 'static,
// {
//     type Response = S::Response;
//     type Error = S::Error;
//     // `BoxFuture` is a type alias for `Pin<Box<dyn Future + Send + 'a>>`
//     type Future = ResponseFuture<S::Future>;

//     fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
//         self.inner.poll_ready(cx)
//     }

//     fn call(&mut self, mut request: Request<Body>) -> Self::Future {
//         let future = self.inner.call(request);
//         Box::pin(async move {
//             let response: Response = future.await?;
//             Ok(response)
//         })
//     }
// }

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "example_print_request_response=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tokio::spawn(server());

    let client = Client::new();

    let app = Router::new().route("/*path", any(proxy_handler)).layer(
        ServiceBuilder::new()
            .layer(Extension(client))
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

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn server() {
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

async fn print_request_response(
    req: Request<Body>,
    next: Next<Body>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let (parts, body) = req.into_parts();
    let bytes = buffer_and_print("request", body).await?;
    let req = Request::from_parts(parts, Body::from(bytes));

    let res = next.run(req).await;

    let (parts, body) = res.into_parts();
    let bytes = buffer_and_print("response", body).await?;
    let res = Response::from_parts(parts, Body::from(bytes));

    Ok(res)
}

async fn buffer_and_print<B>(direction: &str, body: B) -> Result<Bytes, (StatusCode, String)>
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    let bytes = match hyper::body::to_bytes(body).await {
        Ok(bytes) => bytes,
        Err(err) => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("failed to read {} body: {}", direction, err),
            ));
        }
    };

    if let Ok(body) = std::str::from_utf8(&bytes) {
        tracing::debug!("{} body = {:?}", direction, body);
    }

    Ok(bytes)
}
