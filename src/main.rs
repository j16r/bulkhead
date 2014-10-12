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
  id: i32
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
  id: i32,
  name: String,
  created_at: Timespec
}

#[deriving(Decodable)]
struct NewSessionRequest {
  username: String,
  password: String
}

fn authenticate_user(new_session_request : &NewSessionRequest) -> Option<User> {
  let db = db_connect();
  let stmt = db.prepare("SELECT
                          id, name, created_at
                         FROM users
                         WHERE name = $1").unwrap();
  for row in stmt.query(&[&new_session_request.username]).unwrap() {
    return Some(User {
      id: row.get(0u),
      name: row.get(1u),
      created_at: row.get(2u)
    })
  }
  None
}

fn create_session(user: &User) -> Option<Session> {
  let db = db_connect();
  let stmt = db.execute("INSERT INTO sessions (user_id, created_at)
                         VALUES($1, now())",
                        &[&user.id]).unwrap();
  let stmt = db.prepare("SELECT
                          id
                         FROM sessions
                         WHERE id = lastval()").unwrap();
  for row in stmt.query([]).unwrap() {
    return Some(Session {
      id: row.get(0u),
    })
  }
  None
}

fn new_session_handler(req: &mut Request) -> IronResult<Response> {
  let new_session_request : NewSessionRequest = json::decode(req.body.as_slice()).unwrap();

  let user = match authenticate_user(&new_session_request) {
    Some(user) => user,
    None => return Ok(Response::with(status::Unauthorized, "Unauthorized!"))
  };

  let response = SessionResponse{session: create_session(&user).unwrap()};
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
