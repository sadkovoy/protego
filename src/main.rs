extern crate actix;
extern crate actix_web;
extern crate chrono;
extern crate env_logger;
extern crate futures;

mod handlers;
mod state;

use actix_web::{App, middleware, server};

use handlers::{proxy};
use state::{AppState, create_state};


fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("protego");

    let entries = create_state();

    server::new(move || {
        App::with_state(AppState{entries: entries.clone()})
            .middleware(middleware::Logger::default())
            .default_resource(|r| r.with_async(proxy))
    }).bind("0.0.0.0:8000")
        .unwrap()
        .start();

    println!("Started http server: 0.0.0.0:8000");
    let _ = sys.run();
}
