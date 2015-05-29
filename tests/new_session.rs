extern crate hyper;

use std::io::{self, Read};
use hyper::client::Client;
use hyper::client::response::Response;
use std::process::{Command, Child};
use std::thread::sleep_ms;

struct Context {
  server: Child
}

fn setup() -> Context {
  let server = Command::new("target/debug/bulkhead")
    .spawn()
    .unwrap_or_else(|msg| panic!("Failed to launch bulkhead server: {}", msg));

  sleep_ms(100);
  println!("bulkhead server running...");

  Context {server: server}
}

impl Drop for Context {
    fn drop(&mut self) {
        self.server.kill()
            .unwrap_or_else(|msg| panic!("Failed to shut down bulkhead server: {}", msg));
    }
}

fn read_to_string(mut r: Response) -> io::Result<String> {
    let mut s = String::new();
    try!(r.read_to_string(&mut s));
    Ok(s)
}

#[test]
fn new_session_test() {
  let mut ctx = setup();

  let mut client = Client::new();
  let response = client
      .post("http://localtest.me:3000/sessions")
      .body(r#"{"session":{"username": "timmy", "password":"1234"}}"#.as_bytes())
      .send()
      .unwrap();

  assert_eq!(response.status, hyper::Ok);
  assert_eq!(read_to_string(response).unwrap(), r#"{"session":{"id":1}}"#);
}
