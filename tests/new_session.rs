extern crate hyper;

use std::io::Command;
use std::io::timer::Timer;
use std::time::duration::Duration;
use std::io::MemWriter;
use std::io::util::copy;

use hyper::Url;
use hyper::client::Request;

fn setup() {
  spawn(proc() {
    match Command::new("target/bulkhead").spawn() {
      Ok(_) => println!("bulkhead server running..."),
      Err(msg) => fail!("Failed to launch bulkhead server: {}", msg)
    }
  });

  Timer::new().unwrap().sleep(Duration::seconds(1));
}

#[test]
fn new_session_test() {
  setup();

  let new_session_url = Url::parse("http://localtest.me:3000/sessions").unwrap();
  let request = match Request::post(new_session_url) {
    Ok(request) => request,
    Err(err) => fail!("Failed to connect to bulkhead on {}", err)
  };

  let mut stream = match request.start() {
    Ok(stream) => stream,
    Err(err) => fail!("Failed to write to request {}", err)
  };

  stream.write("{\"session\":{\"username\": \"timmy\", \"password\":\"1234\"}}".as_bytes());
  let mut response = match stream.send() {
    Ok(response) => response,
    Err(err) => fail!("Failed to read response {}", err)
  };

  let mut body = MemWriter::new();
  copy(&mut response, &mut body);
  let output = String::from_utf8(body.unwrap()).unwrap();

  assert_eq!(output.as_slice(), "{\"session\":{\"id\":1}}");
}
