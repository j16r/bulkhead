extern crate iron;
extern crate postgres;
extern crate router;
extern crate rustc_serialize;
extern crate time;

use iron::prelude::*;
use router::Router;

mod db;
mod sessions;

fn main() {
    let db = db::connect();
    db::migrate(&db);

    let mut router = Router::new();
    router.post("/sessions", sessions::create);

    Iron::new(router)
        .http("0.0.0.0:3000")
        .unwrap_or_else(|error| panic!("Unable to start server: {}", error));

    println!("Bulkhead ready.");
}
