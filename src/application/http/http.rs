use crate::infrastructure;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Copy, PartialOrd, Ord)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    OPTIONS,
}

pub struct HeaderValue(String);
impl HeaderValue {
    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

pub type Headers = HashMap<String, HeaderValue>;
pub type Params = HashMap<String, String>;

pub struct Status {
    code: u16,
}

impl Status {
    pub fn new(code: u16) -> Self {
        Self { code }
    }
}

impl Status {
    pub fn is_success(&self) -> bool {
        self.code >= 200 && self.code < 300
    }

    pub fn is_client_error(&self) -> bool {
        self.code >= 400 && self.code < 500
    }

    pub fn is_server_error(&self) -> bool {
        self.code >= 500 && self.code < 600
    }
}

pub struct HttpRequest<T: Serialize + Send + Sync> {
    pub method: HttpMethod,
    pub url: String,
    pub params: Option<Params>,
    pub headers: Option<Headers>,
    pub body: Option<T>,
    pub timeout: Option<Duration>,
}

impl<T: Serialize + Send + Sync> HttpRequest<T> {
    pub fn new(
        method: HttpMethod,
        url: String,
        headers: Option<Headers>,
        params: Option<Params>,
        body: Option<T>,
        timeout: Option<Duration>,
    ) -> Self {
        Self {
            method,
            url,
            params,
            headers,
            body,
            timeout,
        }
    }
}

// Helper constructor for requests without body

pub struct HttpResponse<T: DeserializeOwned = ()> {
    pub status: Status,
    pub headers: Headers,
    pub body: Option<T>,
}

impl<T: DeserializeOwned> HttpResponse<T> {
    pub fn new(status: Status, headers: Headers, body: Option<T>) -> Self {
        Self {
            status,
            headers,
            body,
        }
    }
}

#[async_trait::async_trait]
pub trait HttpRequester: Send + Sync + Sized + 'static {
    async fn make_request<T, O>(
        &self,
        req: HttpRequest<T>,
    ) -> infrastructure::Result<HttpResponse<O>>
    where
        T: Serialize + Send + Sync,
        O: DeserializeOwned;
}
