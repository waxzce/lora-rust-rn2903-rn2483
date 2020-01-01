use rn2903::{NvmAddress, Rn2903};
use std::env::args;
use std::process::exit;
use std::thread;
use std::time::Duration;

fn main() {
    let args: Vec<_> = args().collect();
    if args.len() <= 1 {
        eprintln!("rn2903_nvm_set_get <serial port>");
        eprintln!("\tGet, modify, check, and restore the contents of NVM::0x300.");
        exit(1);
    }

    let mut txvr = Rn2903::new_at(&args[1]).expect("Could not open device. Error");
    println!(
        "Successfully connected. Version: {}",
        txvr.system_version()
            .expect("Could not read from device. Error:")
    );

    txvr.system_module_reset().unwrap();
    txvr.transact(b"mac pause").unwrap();
    let addr = NvmAddress::new(0x300);

    txvr.transact(b"sys set pindig GPIO10 1").unwrap();
    let prev = txvr.system_get_nvm(addr).unwrap();
    println!("Previous value: {:#x}", prev);
    txvr.transact(b"sys set pindig GPIO10 0").unwrap();

    txvr.transact(b"sys set pindig GPIO11 1").unwrap();
    txvr.system_set_nvm(addr, 0xAB).unwrap();
    println!("Wrote new value");
    txvr.transact(b"sys set pindig GPIO11 0").unwrap();

    txvr.transact(b"sys set pindig GPIO10 1").unwrap();
    let new = txvr.system_get_nvm(addr).unwrap();
    println!("New value: {:#x}", new);
    txvr.transact(b"sys set pindig GPIO10 0").unwrap();

    txvr.transact(b"sys set pindig GPIO11 1").unwrap();
    txvr.system_set_nvm(addr, prev).unwrap();
    println!("Restored old value");
    txvr.transact(b"sys set pindig GPIO11 0").unwrap();
}
