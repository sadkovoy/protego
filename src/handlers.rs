extern crate actix;
extern crate actix_web;
extern crate env_logger;

use actix_web::{HttpRequest, HttpMessage, HttpResponse, Error};
use futures::{Future};

use state::AppState;
use request::{request_method_builder, update_request_headers};

pub fn proxy(req: HttpRequest<AppState>) -> impl Future<Item=HttpResponse, Error=Error> {
    let _remote_addr = req.peer_addr().unwrap().ip().to_string();

    let uri = req.uri();
    let method = req.method();
    let headers = req.headers();

    let host = "www.wix.com";

    let mut request = request_method_builder(method.to_owned(), format!("https://{}{}", host, uri));
    update_request_headers(&mut request, headers, host);

    request
        .finish()
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
        )
}
