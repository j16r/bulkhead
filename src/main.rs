extern crate bodyparser;
extern crate iron;
extern crate postgres;
extern crate router;
extern crate rustc_serialize;
extern crate time;

use iron::prelude::*;
use iron::status;
use postgres::{Connection, SslMode};
use router::Router;
use rustc_serialize::json;
use time::Timespec;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Session  {
  id: i32
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct SessionResponse {
  session: Session
}

fn db_connect() -> Connection {
  Connection::connect("postgres://bulkhead@localhost",
                      &SslMode::None).unwrap()
}

fn migrate(conn: &Connection) {
  conn.execute("CREATE TABLE IF NOT EXISTS users (
                  id              SERIAL PRIMARY KEY,
                  name            VARCHAR NOT NULL,
                  created_at      TIMESTAMP NOT NULL
                )", &[]).unwrap();
  conn.execute("CREATE TABLE IF NOT EXISTS sessions (
                  id              SERIAL PRIMARY KEY,
                  user_id         int,
                  created_at      TIMESTAMP NOT NULL,

                  CONSTRAINT fk_user FOREIGN KEY (user_id)
                    REFERENCES users (id)
                )", &[]).unwrap();
}

struct User {
  id: i32,
  name: String,
  created_at: Timespec
}

#[derive(RustcDecodable, Clone, Debug)]
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
      id: row.get(0),
      name: row.get(1),
      created_at: row.get(2)
    })
  }
  None
}

fn create_session(user: &User) -> Option<Session> {
  let db = db_connect();
  db.execute("INSERT INTO sessions (user_id, created_at)
              VALUES($1, now())",
             &[&user.id]).ok();
  let stmt = db.prepare("SELECT
                          id
                         FROM sessions
                         WHERE id = lastval()").unwrap();
  for row in stmt.query(&[]).unwrap() {
    return Some(Session {
      id: row.get(0),
    })
  }
  None
}

fn new_session_handler(req: &mut Request) -> IronResult<Response> {
  let new_session_request_result =
      req.get::<bodyparser::Struct<NewSessionRequest>>();

  let new_session_request_option = match new_session_request_result {
      Ok(option) => option,
      Err(error) => return Ok(
          Response::with(
              (status::BadRequest,
               format!("Unable to parse request: {}", error))))
  };

  let new_session_request = match new_session_request_option {
      Some(request) => request,
      None => return Ok(
          Response::with(
              (status::BadRequest,
               "No Content-Type header provided")))
  };

  let user = match authenticate_user(&new_session_request) {
    Some(user) => user,
    None => return Ok(
        Response::with(status::Unauthorized))
  };

  let response = json::encode(
      &SessionResponse{session: create_session(&user).unwrap()})
      .unwrap();
  Ok(Response::with((status::Ok, response)))
}

fn main() {
  let db = db_connect();
  migrate(&db);

  let mut router = Router::new();
  router.post("/sessions", new_session_handler);

  Iron::new(router)
      .http("0.0.0.0:3000")
      .unwrap_or_else(|error| panic!("Unable to start server: {}", error));

  println!("Bulkhead ready.");
}
