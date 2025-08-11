// use std::collections::HashMap;
//
// pub enum HttpMethod {
//     GET,
//     POST,
//     PUT,
//     DELETE,
//     PATCH,
//     OPTIONS,
// }
//
// pub enum HeaderValue {
//     One(String),
//     Many(Vec<String>),
// }
//
// pub type Headers = HashMap<String, HeaderValue>;
// pub type Params = HashMap<String, String>;
//
// struct HttpRequest<Serialize> {
//     method: HttpMethod,
//     url: String,
//     params: Params,
//     headers: Headers,
//     body: Option<Serialize>,
// }
//
// struct HttpResponse<Deserialize> {
//     status: u16,
//     heade    rs: Headers,
//     body: Option<Deserialize>,
// }
//
//
// pub trait HttpRequester {
//     async fn make_request<T, O>(req: HttpRequest<T>) -> HttpResponse<O>;
// }
