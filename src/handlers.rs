extern crate actix;
extern crate actix_web;
extern crate env_logger;

use actix_web::{
    client, http::Method, http::header::HOST,
    HttpRequest, HttpMessage, HttpResponse, Error
};
use chrono::prelude::*;
use futures::{future::ok as future_ok, Future};

use state::{AppState};

pub fn proxy(req: HttpRequest<AppState>) -> impl Future<Item = HttpResponse, Error = Error> {
    let mut entries = req.state().entries.lock().unwrap();

    let remote_addr = req.peer_addr().unwrap().ip().to_string();

    let uri = req.uri();
    let method = req.method();
    let headers = req.headers();

    let host = "vod.wix.com";

    let request_func: fn(String) -> client::ClientRequestBuilder = match method {
        &Method::GET => client::ClientRequest::get,
        &Method::HEAD => client::ClientRequest::head,
        &Method::POST => client::ClientRequest::post,
        &Method::PUT => client::ClientRequest::put,
        &Method::DELETE => client::ClientRequest::delete,
        _ => client::ClientRequest::get
    };


    request_func(format!("https://{}{}", host, uri))
        .finish()
        .unwrap()
        .send()
        .map_err(Error::from)
        .and_then(
            |resp| resp.body()
                .from_err()
                .and_then(|body| {
                    Ok(HttpResponse::Ok().body(body))
                })
        )
}
