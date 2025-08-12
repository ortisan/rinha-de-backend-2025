use crate::infrastructure;
use std::collections::HashMap;
use std::time::Duration;

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

pub struct HttpRequest<Serialize> {
    pub method: HttpMethod,
    pub url: String,
    pub params: Params,
    pub headers: Headers,
    pub body: Option<Serialize>,
    pub timeout: Option<Duration>,
}

pub struct HttpResponse<Deserialize> {
    pub status: Status,
    pub headers: Headers,
    pub body: Option<Deserialize>,
}

pub trait HttpRequester {
    async fn make_request<T, O>(
        &self,
        req: HttpRequest<T>,
    ) -> infrastructure::Result<HttpResponse<O>>;
}
