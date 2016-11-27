#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;

extern crate iron;
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate router;
extern crate serde_json;
extern crate time;

mod db;
mod sessions;

use db::DbPool;
use iron::prelude::*;
use router::Router;

fn main() {
    let db_pool_middleware = DbPool::new().unwrap();

    let mut session_handler = Chain::new(sessions::create);
    session_handler.link_before(db_pool_middleware);

    let mut router = Router::new();
    router.post("/sessions", session_handler, "post");

    Iron::new(router)
        .http("0.0.0.0:3000")
        .unwrap_or_else(|error| panic!("Unable to start server: {}", error));

    println!("Bulkhead ready.");
}
