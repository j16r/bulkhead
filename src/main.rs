extern crate iron;
extern crate router;
extern crate serialize;

use std::io::net::ip::Ipv4Addr;
use serialize::json;

use iron::{Iron, Request, Response, IronResult};
use router::{Router, Params};
use iron::status;


#[deriving(Decodable, Encodable)]
pub struct Session  {
  id: u8
}

#[deriving(Decodable, Encodable)]
pub struct SessionResponse {
  session: Session
}

fn new_session_handler(_: &mut Request) -> IronResult<Response> {
  let response = SessionResponse{session: Session{id: 1}};
  Ok(Response::with(status::Ok, json::encode(&response)))
}

fn main() {
  let mut router = Router::new();
  router.post("/sessions", new_session_handler);

  Iron::new(router).listen(Ipv4Addr(127, 0, 0, 1), 3000);
}
