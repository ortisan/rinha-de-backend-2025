use crate::application::http::http::{
    HeaderValue, HttpMethod, HttpRequest, HttpRequester, HttpResponse, Status,
};
use crate::infrastructure;
use reqwest::{Client, Method, Response, StatusCode};
use std::sync::Arc;

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
    async fn make_request<T, O>(
        &self,
        req: HttpRequest<T>,
    ) -> infrastructure::Result<HttpResponse<O>> {
        let mut request_builder = self.http_client.request(Method::from(req.method), req.url);
        if (req.timeout).is_some() {
            request_builder = request_builder.timeout(req.timeout.unwrap());
        }
        if req.headers.len() > 0 {
            for (key, value) in req.headers.iter() {
                request_builder = request_builder.header(key, {
                    let value_str = value.as_str();
                    reqwest::header::HeaderValue::from_str(&value_str).unwrap()
                });
            }
        }
        let response_result = request_builder.send().await;
        match response_result {
            Ok(response) => Ok(HttpResponse::from(response)),
            Err(error) => Err(error.into()),
        }
    }
}

impl<B> From<reqwest::Response> for HttpResponse<B> {
    fn from(value: Response) -> Self {
        let mut headers = std::collections::HashMap::new();
        for (key, value) in value.headers() {
            headers.insert(key.to_string(), HeaderValue::from(value.clone()));
        }
        let _ = value.headers();
        HttpResponse::<B> {
            status: Status::from(value.status()),
            headers,
            body: None,
        }
    }
}

impl From<StatusCode> for Status {
    fn from(value: StatusCode) -> Status {
        Status::new(value.as_u16())
    }
}

impl From<reqwest::header::HeaderValue> for HeaderValue {
    fn from(value: reqwest::header::HeaderValue) -> Self {
        HeaderValue::new(String::from(value.to_str().unwrap()))
    }
}
