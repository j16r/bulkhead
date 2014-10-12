extern crate iron;
extern crate router;
extern crate serialize;
extern crate postgres;
extern crate time;

use std::io::net::ip::Ipv4Addr;
use serialize::json;

use iron::{Iron, Request, Response, IronResult};
use router::{Router, Params};
use iron::status;
use postgres::{PostgresConnection, NoSsl};
use postgres::types::ToSql;
use time::Timespec;

#[deriving(Decodable, Encodable)]
pub struct Session  {
  id: u8
}

#[deriving(Decodable, Encodable)]
pub struct SessionResponse {
  session: Session
}

fn db_connect() -> PostgresConnection {
  PostgresConnection::connect("postgres://bulkhead@localhost",
                              &NoSsl).unwrap()
}

fn migrate(conn: &PostgresConnection) {
  conn.execute("CREATE TABLE IF NOT EXISTS users (
                  id              SERIAL PRIMARY KEY,
                  name            VARCHAR NOT NULL,
                  created_at      TIMESTAMP NOT NULL
                )", []).unwrap();
  conn.execute("CREATE TABLE IF NOT EXISTS sessions (
                  id              SERIAL PRIMARY KEY,
                  user_id         int,
                  created_at      TIMESTAMP NOT NULL,

                  CONSTRAINT fk_user FOREIGN KEY (user_id)
                    REFERENCES users (id)
                )", []).unwrap();
}

struct User {
  id: int,
  name: String,
  created_at: Timespec
}

#[deriving(Decodable)]
struct NewSessionRequest {
  username: String,
  password: String
}

fn authenticate_user(new_session_request : &NewSessionRequest) -> Option<User> {
  //let user = db.prepare("SELECT id, name, created_at FROM users").unwrap();
  None
}

fn new_session_handler(req: &mut Request) -> IronResult<Response> {
  let new_session_request : NewSessionRequest = json::decode(req.body.as_slice()).unwrap();

  let user = match authenticate_user(&new_session_request) {
    Some(_) => (),
    None => return Ok(Response::with(status::Unauthorized, "Unauthorized!"))
  };

  let response = SessionResponse{session: Session{id: 1}};
  Ok(Response::with(status::Ok, json::encode(&response)))
}

fn main() {
  let db = db_connect();
  migrate(&db);

  let mut router = Router::new();
  router.post("/sessions", new_session_handler);

  let mut server = Iron::new(router);
  server.listen(Ipv4Addr(127, 0, 0, 1), 3000);
  println!("Bulkhead ready.");
}
