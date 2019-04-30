# TimeStamp 

Rust project

Display time durations.

Example

```rust
extern crate time_stamp;
use std::{thread, time};

fn main() {
    let ts = time_stamp::TimeStamp::new(10);
    ts.stamp();
    thread::sleep(time::Duration::from_secs(1));
    ts.stamp();
    thread::sleep(time::Duration::from_secs(1));
    ts.stamp();
    thread::sleep(time::Duration::from_secs(1));
    ts.stamp();
    ts.print();
}
```