use crate::application::http::http::{HttpMethod, HttpRequest, HttpRequester, HttpResponse};
use reqwest::{Client, Method};
use std::sync::Arc;
use std::time::Duration;

struct Reqwest {
    http_client: Arc<Client>,
}

impl Reqwest {
    fn new(http_client: Arc<Client>) -> Self {
        Reqwest { http_client }
    }
}

impl From<HttpMethod> for Method {
    fn from(value: HttpMethod) -> Self {
        match value {
            HttpMethod::DELETE => Method::DELETE,
            HttpMethod::GET => Method::GET,
            HttpMethod::POST => Method::POST,
            HttpMethod::PUT => Method::PUT,
            HttpMethod::PATCH => Method::PATCH,
            HttpMethod::OPTIONS => Method::OPTIONS,
            _ => panic!("Invalid HTTP method"),
        }
    }
}

impl HttpRequester for Reqwest {
    async fn make_request<T, O>(&self, req: HttpRequest<T>) -> HttpResponse<O> {
        let mut request_builder = self.http_client.request(Method::from(req.method), req.url);
        if (req.timeout).is_some() {
            request_builder = request_builder.timeout(req.timeout.unwrap());
        }
        if req.headers.len() > 0 {
            request_builder = request_builder.headers(req.timeout.unwrap());
        }
        let response = request_builder.send().await?;

    }
}
