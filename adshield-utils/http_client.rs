pub use http::Uri;
pub use reqwest::*;

static GLOBAL_CLIENT: once_cell::sync::OnceCell<Client> = once_cell::sync::OnceCell::new();

pub fn init_with_proxy(proxy_uri: String) {
    if GLOBAL_CLIENT.get().is_some() {
        panic!("Global http client is already initialized!");
    }

    GLOBAL_CLIENT.get_or_init(|| {
        ClientBuilder::new()
            .proxy(Proxy::all(proxy_uri).unwrap())
            .build()
            .unwrap()
    });
}

pub fn request(method: Method, url: impl IntoUrl) -> RequestBuilder {
    GLOBAL_CLIENT.get_or_init(Client::new).request(method, url)
}
