extern crate hyper;

mod support;

use hyper::client::Client;
use hyper::header::ContentType;
use support::{setup, body};

#[test]
fn new_session_test() {
    setup();

    let mut client = Client::new();
    let response = client
        .post("http://localtest.me:3000/sessions")
        .header(ContentType::json())
        .body(r#"{"username": "timmy", "password": "1234"}"#.as_bytes())
        .send()
        .unwrap();

    assert_eq!(response.status, hyper::Ok);
    assert_eq!(body(response), r#"{"session":{"id":1}}"#);
}

#[test]
fn invalid_request_body_syntax_test() {
    setup();

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
    setup();

    let mut client = Client::new();
    let response = client
        .post("http://localtest.me:3000/sessions")
        .body(r#"{}"#.as_bytes())
        .send()
        .unwrap();

    assert_eq!(response.status, hyper::BadRequest);
}
