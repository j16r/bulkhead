extern crate hyper;

use std::io::Command;
use std::io::timer::Timer;
use std::time::duration::Duration;

use hyper::Url;
use hyper::client::Request;

fn setup() {
  spawn(proc() {
    match Command::new("target/bulkhead").spawn() {
      Ok(_) => println!("bulkhead server running..."),
      Err(msg) => fail!("Failed to launch bulkhead server: {}", msg)
    }

    Timer::new().unwrap().sleep(Duration::milliseconds(100));
  })
}

#[test]
fn new_session_test() {
  setup();

  let new_session_url = Url::parse("http://localtest.me:3000/sessions").unwrap();
  let req = Request::post(new_session_url);
  match req {
    Ok(_) => {;},
    Err(err) => fail!("Failed to connect to bulkhead on {}", err)
  };
}
