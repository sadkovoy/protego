extern crate actix;
extern crate actix_web;
extern crate chrono;
extern crate env_logger;
extern crate futures;
extern crate actix_redis;
#[macro_use] extern crate redis_async;


mod handlers;
mod state;
mod request;

use std::sync::Arc;

use actix_web::{App, middleware, server};
use actix_redis::RedisActor;


use handlers::{proxy};
use state::{AppState, prepare_entries};


fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("protego");

    let entries = prepare_entries();  // for in-memory limiter storage

    server::new(move || {
        let redis = Arc::new(RedisActor::start("redis:6379"));
        App::with_state(AppState{entries: entries.clone(), redis})
            .middleware(middleware::Logger::default())
            .default_resource(|r| r.with_async(proxy))
    }).bind("0.0.0.0:8000")
        .unwrap()
        .start();

    println!("Started proxy server: 0.0.0.0:8000");
    let _ = sys.run();
}
