use crate::application::http::http::{
    HeaderValue, Headers, HttpMethod, HttpRequest, HttpRequester, HttpResponse, Status,
};
use crate::infrastructure;
use crate::infrastructure::utils::is_unit_type;
use reqwest::{Client, Method, StatusCode};
use std::convert::From;
use std::sync::Arc;

pub struct HttpReqwest {
    http_client: Arc<Client>,
}

impl HttpReqwest {
    pub fn new(http_client: Arc<Client>) -> Self {
        HttpReqwest { http_client }
    }
}

#[async_trait::async_trait]
impl HttpRequester for HttpReqwest {
    async fn make_request<I, O>(
        &self,
        req: HttpRequest<I>,
    ) -> infrastructure::Result<HttpResponse<O>>
    where
        I: serde::Serialize + Send + Sync,
        O: serde::de::DeserializeOwned,
    {
        let mut request_builder = self.http_client.request(Method::from(req.method), req.url);
        if req.headers.is_some() {
            for (key, value) in req.headers.unwrap().iter() {
                request_builder = request_builder.header(key, {
                    let value_str = value.as_str();
                    reqwest::header::HeaderValue::from_str(&value_str).unwrap()
                });
            }
        }
        if req.body.is_some() {
            request_builder = request_builder.json(&req.body.unwrap());
        }
        if (req.timeout).is_some() {
            request_builder = request_builder.timeout(req.timeout.unwrap());
        }
        // TODO ADD PARAMETERS
        let response = request_builder.send().await?;
        let headers: Headers = response
            .headers()
            .iter()
            .map(|h| (h.0.to_string(), HeaderValue::from(h.1.clone())))
            .collect();
        let status = Status::from(response.status());
        if is_unit_type::<O>() {
            return Ok(HttpResponse::new(status, headers, None));
        }
        let body: O = response.json().await?;
        Ok(HttpResponse::new(status, headers, Some(body)))
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
        }
    }
}

impl From<StatusCode> for Status {
    fn from(value: StatusCode) -> Self {
        Status::new(value.as_u16())
    }
}

impl From<reqwest::header::HeaderValue> for HeaderValue {
    fn from(value: reqwest::header::HeaderValue) -> Self {
        HeaderValue::new(String::from(value.to_str().unwrap()))
    }
}

// impl<B: serde::de::DeserializeOwned> From<&Response> for HttpResponse<B> {
//     fn from(response: &Response) -> Self {
//         let mut headers = std::collections::HashMap::new();
//         for (key, value) in response.headers() {
//             headers.insert(key.to_string(), HeaderValue::from(value.clone()));
//         }
//         let _ = response.headers();
//         let res: Result<B> =  response.json().await;
//
//         HttpResponse::<B> {
//             status: Status::from(response.status()),
//             headers,
//             body: None,
//         }
//     }
// }
