pub mod handler;

pub type Client = hyper::client::Client<hyper::client::HttpConnector, hyper::Body>;
