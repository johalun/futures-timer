extern crate futures;
extern crate futures_timer;

use std::time::{Instant, Duration};

use futures::prelude::*;
use futures_timer::Sleep;

#[test]
fn smoke() {
    let dur = Duration::from_millis(10);
    let start = Instant::now();
    let timeout = Sleep::new(dur);
    timeout.wait().unwrap();
    assert!(start.elapsed() >= (dur / 2));
}

#[test]
fn two() {
    let dur = Duration::from_millis(10);
    let timeout = Sleep::new(dur);
    timeout.wait().unwrap();
    let timeout = Sleep::new(dur);
    timeout.wait().unwrap();
}
