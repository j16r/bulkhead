extern crate bodyparser;

use iron::prelude::*;
use iron::status;
use rustc_serialize::json;
use time::Timespec;

use db;

struct User {
    id: i32,
    name: String,
    created_at: Timespec
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Session  {
    id: i32
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct SessionResponse {
    session: Session
}

#[derive(RustcDecodable, Clone, Debug)]
struct NewSessionRequest {
    username: String,
    password: String
}


fn authenticate_user(new_session_request : &NewSessionRequest) -> Option<User> {
    let db = db::connect();
    let stmt = db.prepare("SELECT id, name, created_at
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
    let db = db::connect();
    db.execute("INSERT INTO sessions (user_id, created_at)
                VALUES($1, now())",
                &[&user.id]).ok();
    let stmt = db.prepare("SELECT id
                           FROM sessions
                           WHERE id = lastval()").unwrap();
    for row in stmt.query(&[]).unwrap() {
        return Some(Session {
            id: row.get(0),
        })
    }
    None
}

pub fn create(req: &mut Request) -> IronResult<Response> {
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
