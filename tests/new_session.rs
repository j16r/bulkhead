extern crate hyper;

use std::thread::Thread;
use std::process::Command;
//use std::time::duration::Duration;

use hyper::Client;

struct Context {
  server: Thread
}

fn setup() -> Context {
  let server = Command::new("target/bulkhead").spawn();
        //.unwrap_or_else(|msg| panic!("Failed to launch bulkhead server: {}", msg));

  //Timer::new().unwrap().sleep(Duration::seconds(2));

  Context {server: server}
}

fn teardown(context: &mut Context) {
  //context.server.kill();
}

#[test]
fn new_session_test() {
  let mut ctx = setup();

  let mut client = Client::new();
  let mut response = client
    .post("http://localtest.me:3000/sessions")
    .body("{\"username\": \"timmy\", \"password\":\"1234\"}".as_bytes())
    .send()
    .unwrap();

  assert_eq!(response.read_to_string().unwrap(), "{\"session\":{\"id\":1}}");

  //teardown(&ctx);
}
