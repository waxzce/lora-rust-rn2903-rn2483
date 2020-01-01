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

For instance, here is a simple program which dumps all LoRa packets received.

```rust
use rn2903::{Rn2903, ModulationMode};

fn main() {
    let mut txvr = Rn2903::new_at("/dev/ttyUSB0")
        .expect("Could not open device. Error");
    txvr.mac_pause().unwrap();
    txvr.radio_set_modulation_mode(ModulationMode::LoRa).unwrap();
    loop {
        if let Some(packet) = txvr.radio_rx(65535).unwrap() {
            println!("{:?}", packet);
        }
    }
}
```

## Module Documentation

This repository reproduces the relevant documents for the RN2903 module at
[command_reference-40001811B.pdf](docu/command_reference-40001811B.pdf) and
[datasheet-DS5000239H.pdf](docu/datasheet-DS5000239H.pdf).

