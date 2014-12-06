extern crate hyper;

use std::io::Process;
use std::io::Command;
use std::io::timer::Timer;
use std::time::duration::Duration;

use hyper::Url;
use hyper::client::Request;

struct Context {
  server: Process
}

fn setup() -> Context {
  let server = Command::new("target/bulkhead")
    .spawn()
    .unwrap_or_else(|msg| panic!("Failed to launch bulkhead server: {}", msg));

  Timer::new().unwrap().sleep(Duration::seconds(1));
  println!("bulkhead server running...");

  Context {server: server}
}

fn teardown(context: &mut Context) {
  //context.server.kill();
}

#[test]
fn new_session_test() {
  let mut ctx = setup();

  let new_session_url = Url::parse("http://localtest.me:3000/sessions").unwrap();
  let request = Request::post(new_session_url)
    .unwrap_or_else(|error| panic!("Failed to connect to bulkhead on {}", error));

  let mut stream = request.start()
    .unwrap_or_else(|error| panic!("Failed to write to request {}", error));

  stream.write("{\"session\":{\"username\": \"timmy\", \"password\":\"1234\"}}".as_bytes())
    .unwrap();

  let mut response = stream.send()
    .unwrap_or_else(|error| panic!("Failed to read response {}", error));

  assert_eq!(response.read_to_string().unwrap().as_slice(), "{\"session\":{\"id\":1}}");

  //teardown(&ctx);
}
