use rn2903::Rn2903;
use std::env::args;
use std::process::exit;
use std::thread;
use std::time::Duration;

fn main() {
    let args: Vec<_> = args().collect();
    if args.len() <= 1 {
        eprintln!("rn2903_lora_packet_rx <serial port>");
        eprintln!("\tRecieve LoRa packets and print their corresponding hex values.");
        exit(1);
    }

    let mut txvr = Rn2903::new_at(&args[1]).expect("Could not open device. Error");
    println!(
        "Successfully connected. Version: {}",
        txvr.system_version()
            .expect("Could not read from device. Error:")
    );

    txvr.mac_pause().unwrap();
    txvr.transact(b"sys set pindig GPIO10 0").unwrap();
    txvr.transact(b"radio rx 0").unwrap();
    loop {
        println!("{:?}", txvr.read_line().unwrap());
        txvr.transact(b"sys set pindig GPIO10 1").unwrap();
        thread::sleep(Duration::from_millis(100));
        txvr.transact(b"sys set pindig GPIO10 0").unwrap();
        txvr.transact(b"radio rx 0").unwrap();
    }
}
