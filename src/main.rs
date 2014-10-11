extern crate iron;
extern crate router;

use std::io::net::ip::Ipv4Addr;

use iron::{Iron, Request, Response, IronResult};
use router::{Router, Params};
use iron::status;

fn new_session_handler(_: &mut Request) -> IronResult<Response> {
  Ok(Response::with(status::Ok, "AUTHZ!"))
}

fn main() {
  let mut router = Router::new();
  router.post("/sessions", new_session_handler);

  Iron::new(router).listen(Ipv4Addr(127, 0, 0, 1), 3000);
}
