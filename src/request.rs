use actix_web::client::{ClientRequestBuilder, ClientRequest};
use actix_web::http::Method;
use actix_web::http::HeaderMap;
use actix_web::http::header::HOST;

pub fn request_method_builder(method: Method) -> ClientRequestBuilder {
    let mut builder = ClientRequest::build();
    builder.method(method);
    builder
}

pub fn set_request_headers(request_builder: &mut ClientRequestBuilder, headers: &HeaderMap, host: &str) {
    for (header_name, header_value) in headers {
        request_builder.header(header_name, header_value.to_str().unwrap());
    }
    request_builder.set_header(HOST, host);
}

pub fn set_request_uri(request_builder: &mut ClientRequestBuilder, uri: String) {
    request_builder.uri(uri);
}