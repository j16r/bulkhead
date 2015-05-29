extern crate hyper;

mod support;

use hyper::client::Client;
use hyper::client::response::Response;
use hyper::header::ContentType;
use support::{setup, read_to_string};

#[test]
fn new_session_test() {
  let _ = setup();

  let mut client = Client::new();
  let response = client
      .post("http://localtest.me:3000/sessions")
      .header(ContentType::json())
      .body(r#"{"username": "timmy", "password": "1234"}"#.as_bytes())
      .send()
      .unwrap();

  assert_eq!(response.status, hyper::Ok);
  assert_eq!(read_to_string(response).unwrap(), r#"{"session":{"id":1}}"#);
}

#[test]
fn invalid_request_body_syntax_test() {
  let _ = setup();

  let mut client = Client::new();
  let response = client
      .post("http://localtest.me:3000/sessions")
      .header(ContentType::json())
      .body(r#"{"#.as_bytes())
      .send()
      .unwrap();

  assert_eq!(response.status, hyper::BadRequest);
}

#[test]
fn missing_content_type_header_test() {
  let _ = setup();

  let mut client = Client::new();
  let response = client
      .post("http://localtest.me:3000/sessions")
      .body(r#"{}"#.as_bytes())
      .send()
      .unwrap();

  assert_eq!(response.status, hyper::BadRequest);
}
