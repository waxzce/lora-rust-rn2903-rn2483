# RN2903
## A Rusty interface for the RN2903 LoRa module's serial protocol

The RN2903 is a LoRa and FSK transciever for the 915MHz ISM band, commonly used in USB
devices like the LoStik.

This crate provides a safe, idiomatic interface using cross-platform native serial
functionality via `serialport`. This supports, for instance, a LoStik connected to a USB
TTY or virtual COM port, or a RN2903 connected via a TTL serial interface.

This crate is available under the GNU General Public License, version 3.0 only, and does
not directly depend on unstable crates.

## Example

For instance, here is a simple program which blinks the LoStik's LED using the RN2903's
GPIO functionality.

```rust
use rn2903::Rn2903;
use std::time::Duration;
use std::thread;

fn main() {
    let mut txvr = Rn2903::new_at("/dev/ttyUSB0")
        .expect("Could not open device. Error");
    loop {
        txvr.transact(b"sys set pindig GPIO10 0").unwrap();
        thread::sleep(Duration::from_millis(1000));
        txvr.transact(b"sys set pindig GPIO10 1").unwrap();
        thread::sleep(Duration::from_millis(1000));
    }
}
```

## Module Documentation

This repository reproduces the relevant documents for the RN2903 module at
[command_reference-40001811B.pdf](docu/command_reference-40001811B.pdf) and
[datasheet-DS5000239H.pdf](docu/datasheet-DS5000239H.pdf).

