extern crate hyper;

use self::hyper::client::response::Response;
use std::io::Read;
use std::process::Command;
use std::sync::{Once, ONCE_INIT};
use std::thread::sleep_ms;

static START: Once = ONCE_INIT;

pub fn setup() {
    START.call_once(|| {
        Command::new("target/debug/bulkhead")
            .spawn()
            .unwrap_or_else(|msg| panic!("Failed to launch bulkhead server: {}", msg));

        sleep_ms(100);
        println!("bulkhead server running...");
    });
}

pub fn body(mut r: Response) -> String {
    let mut s = String::new();
    r.read_to_string(&mut s).unwrap();
    s
}
