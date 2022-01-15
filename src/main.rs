use sysinfo::{System, SystemExt, ProcessorExt};
use std::process::Command;
use std::{thread, time};

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();
    const BACKGROUND: &str = "562500";
    const START_COLOR: &str = "210000";
    let output1 = Command::new("g910-led").args(["-a", START_COLOR]).output().expect("Failed to execute");
    thread::sleep(time::Duration::from_secs(1));
    let output2 = Command::new("g910-led").args(["-a", BACKGROUND]).output().expect("Failed to execute");
    for core in sys.processors() {
        println!("Core: {} Usage: {}", core.name(), core.cpu_usage());
    }
    println!("{:?}\n{:?}", output1, output2);
}
