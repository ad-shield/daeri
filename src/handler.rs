use crate::Client;
// use axum::handler::Handler;
use axum::response::Response;
use axum::Extension;
use hyper::{Body, Request, Uri};

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
