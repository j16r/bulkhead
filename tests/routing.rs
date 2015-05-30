extern crate hyper;

mod support;

use hyper::client::Client;
use support::setup;

#[test]
fn invalid_route_test() {
  setup();

  let mut client = Client::new();
  let response = client
      .post("http://localtest.me:3000/fhbwgias")
      .send()
      .unwrap();

  assert_eq!(response.status, hyper::NotFound);
}
