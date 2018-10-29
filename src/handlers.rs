extern crate actix;
extern crate actix_web;
extern crate env_logger;

use std::sync::Arc;

use actix::Addr;
use actix_redis::{Command, RedisActor};
use actix_web::{AsyncResponder, Body, HttpRequest, HttpMessage, HttpResponse, http::StatusCode, Error};
use futures::{Future, future::ok as future_ok, future::err as future_error, future::lazy, Stream};
use redis_async::resp::FromResp;

use state::AppState;
use request::{request_method_builder, set_request_headers, set_request_uri};


fn check_if_allowed(redis: Arc<Addr<RedisActor>>, remote_addr: String) -> impl Future<Item=(), Error=()> {
    redis
        .send(Command(resp_array!["INCR", remote_addr]))
        .then(move |res| {
            let response = res.unwrap();
            let counter = i64::from_resp(response.unwrap()).unwrap();
            if counter > 150 {  // dummy value
                future_error(())
            } else {
                future_ok(())
            }
        })
}

pub fn proxy(req: HttpRequest<AppState>) -> impl Future<Item=HttpResponse, Error=Error> {
    let redis = req.state().redis.clone();
    let remote_addr = req.peer_addr().unwrap().ip().to_string();

    check_if_allowed(redis, remote_addr).then(
        move |allowed| {
            if allowed.is_err() {
                return lazy(move || {
                    Ok(HttpResponse::new(StatusCode::TOO_MANY_REQUESTS))
                }).responder();
            }


            let uri = req.uri();
            let method = req.method();
            let headers = req.headers();
            let body = req.payload();
            let host = "wix.com";

            let mut request = request_method_builder(method.to_owned());
            set_request_uri(&mut request, format!("https://{}{}", host, uri));
            set_request_headers(&mut request, headers, host);

            request
                .body(Body::Streaming(Box::new(body.map_err(|e| e.into()))))
                .unwrap()
                .send()
                .map_err(Error::from)
                .and_then(
                    |resp| {
                        resp
                            .body()
                            .limit(10_485_760)  // 10 Mb
                            .from_err()
                            .and_then(move |body| {
                                let mut response = HttpResponse::Ok();
                                for (header_name, header_value) in resp.headers() {
                                    response.header(header_name, header_value.to_str().unwrap());
                                }
                                Ok(response.body(body))
                            })
                    }
                ).responder()
        }
    )
}
