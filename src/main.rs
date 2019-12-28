use rn2903::Rn2903;

fn main() {
    let mut device = Rn2903::new_at("/dev/ttyUSB0").expect("Could not open device. Error");
    println!("Successfully connected. Version: {}", device.system_version().expect("Could not read from device. Error:"));
}
