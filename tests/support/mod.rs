extern crate hyper;

use self::hyper::client::response::Response;
use std::io::{self, Read};
use std::process::{Command, Child};
use std::thread::sleep_ms;

pub struct Context {
  server: Child
}

pub fn setup() -> Context {
  let server = Command::new("target/debug/bulkhead")
    .spawn()
    .unwrap_or_else(|msg| panic!("Failed to launch bulkhead server: {}", msg));

  sleep_ms(100);
  println!("bulkhead server running...");

  Context {server: server}
}

impl Drop for Context {
    fn drop(&mut self) {
        // self.server.kill()
        //     .unwrap_or_else(|msg| panic!("Failed to shut down bulkhead server: {}", msg));
    }
}

pub fn read_to_string(mut r: Response) -> io::Result<String> {
    let mut s = String::new();
    try!(r.read_to_string(&mut s));
    Ok(s)
}
