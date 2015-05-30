extern crate hyper;

use self::hyper::client::response::Response;
use std::io::{self, Read};
use std::process::{Command, Child};
use std::sync::{Once, ONCE_INIT};
use std::thread::sleep_ms;

static START: Once = ONCE_INIT;

pub fn setup() {
  START.call_once(|| {
    let server = Command::new("target/debug/bulkhead")
        .spawn()
        .unwrap_or_else(|msg| panic!("Failed to launch bulkhead server: {}", msg));

    sleep_ms(100);
    println!("bulkhead server running...");
  });
}

pub fn read_to_string(mut r: Response) -> io::Result<String> {
    let mut s = String::new();
    try!(r.read_to_string(&mut s));
    Ok(s)
}
