extern crate iron;
extern crate router;
extern crate serialize;
extern crate postgres;
extern crate time;

use serialize::json;
use iron::{Iron, IronResult, Request, Response};
use iron::modifier::Set;
use iron::response::modifiers::{Status, Body};
use iron::status;
use router::Router;
use postgres::{Connection, SslMode};
use time::Timespec;

#[deriving(Decodable, Encodable)]
pub struct Session  {
  id: i32
}

#[deriving(Decodable, Encodable)]
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
  db.execute("INSERT INTO sessions (user_id, created_at)
              VALUES($1, now())",
             &[&user.id]).ok();
  let stmt = db.prepare("SELECT
                          id
                         FROM sessions
                         WHERE id = lastval()").unwrap();
  for row in stmt.query(&[]).unwrap() {
    return Some(Session {
      id: row.get(0u),
    })
  }
  None
}

fn new_session_handler(req: &mut Request) -> IronResult<Response> {
  let new_session_request : NewSessionRequest = json::decode(req.body.to_string().as_slice()).unwrap();

  let user = match authenticate_user(&new_session_request) {
    Some(user) => user,
    None => return Ok(
        Response::new()
            .set(Status(status::Unauthorized))
            .set(Body("Unauthorized!")))
  };

  let response = SessionResponse{session: create_session(&user).unwrap()};
  Ok(Response::new()
     .set(Status(status::Ok))
     .set(Body(json::encode(&response))))
}

fn main() {
  let db = db_connect();
  migrate(&db);

  let mut router = Router::new();
  router.post("/sessions", new_session_handler);

  Iron::new(router)
      .listen("0.0.0.0:3000")
      .unwrap_or_else(|error| panic!("Failed to listen {}", error));

  println!("Bulkhead ready.");
}
