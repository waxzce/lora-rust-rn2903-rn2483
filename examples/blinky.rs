use rn2903::Rn2903;
use std::env::args;
use std::process::exit;
use std::thread;
use std::time::Duration;

fn main() {
    let args: Vec<_> = args().collect();
    if args.len() <= 1 {
        eprintln!("rn2903_blinky <serial port>");
        eprintln!("\tReset the module and toggle pin 0b10 on and off.");
        eprintln!("\tThis corresponds to the blue user LED on the LoStik.");
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

    loop {
        txvr.transact(b"sys set pindig GPIO10 1").unwrap();
        thread::sleep(Duration::from_millis(1000));
        txvr.transact(b"sys set pindig GPIO10 0").unwrap();
        thread::sleep(Duration::from_millis(1000));
    }
}
