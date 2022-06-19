use crate::Client;
use axum::body::Bytes;
// use axum::handler::Handler;
use axum::response::IntoResponse;
use axum::Extension;
use axum::{middleware::Next, response::Response};
use hyper::{Body, Request, StatusCode, Uri};

// #[derive(Clone)]
// pub struct Proxy {

// }

// impl Handler<()> for Proxy {
//     type Future = Pin<Box<dyn Future<Output = Response> + Send>>;

//     fn call(self, req: Request<HyperBody>) -> Self::Future {
//         Box::pin(async move {
//             let mut req = RequestParts::new(req);
//             match router.call(&req).await {
//                 Ok(v) => {
//                     *req.uri_mut() = v.uri;
//                     req.headers_mut()
//                         .insert("Host", v.hostname.try_into().unwrap());
//                 }
//                 Err(e) => return Ok(e.into_response().map(Either::Rhs)),
//             };

//             client.request(req).await.unwrap()
//             res.into_response()
//         })
//     }
// }

pub async fn proxy_handler(
    Extension(client): Extension<Client>,
    // NOTE: Make sure to put the request extractor last because once the request
    // is extracted, extensions can't be extracted anymore.
    mut req: Request<Body>,
) -> Response<Body> {
    let path = req.uri().path();
    let path_query = req
        .uri()
        .path_and_query()
        .map(|v| v.as_str())
        .unwrap_or(path);

    let uri = format!("http://127.0.0.1:3000{}", path_query);

    *req.uri_mut() = Uri::try_from(uri).unwrap();

    client.request(req).await.unwrap()
}

pub(crate) async fn print_request_response(
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

pub(crate) async fn buffer_and_print<B>(
    direction: &str,
    body: B,
) -> Result<Bytes, (StatusCode, String)>
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
