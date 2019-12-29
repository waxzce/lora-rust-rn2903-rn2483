use rn2903::{Rn2903, bytes_to_string};
use std::thread;
use std::time::Duration;

fn main() {
    let mut txvr = Rn2903::new_at("/dev/ttyUSB0").expect("Could not open device. Error");
    println!(
        "Successfully connected. Version: {}",
        txvr
            .system_version()
            .expect("Could not read from device. Error:")
    );

    dbg!(bytes_to_string(&txvr.transact(b"sys set pindig GPIO10 1").unwrap()));
    thread::sleep(Duration::from_millis(200));
    dbg!(bytes_to_string(&txvr.transact(b"sys set pindig GPIO10 0").unwrap()));
}
