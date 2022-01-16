use sysinfo::{System, SystemExt, ProcessorExt};
use std::process::Command;
use std::{thread, time};


fn get_color(percent: f32) -> String {
    let percent = percent / 100.0;
    let r = (percent * 255.0) as i32;
    let g =  (110.0 - (percent * 110.0)) as i32;
    format!("{:02X}{:02X}00", r, g)
}

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();
    const BACKGROUND: &str = "562500";
    const START_COLOR: &str = "210000";
    let output1 = Command::new("g910-led").args(["-a", START_COLOR]).output().expect("Failed to execute");
    thread::sleep(time::Duration::from_secs(1));
    let output2 = Command::new("g910-led").args(["-a", BACKGROUND]).output().expect("Failed to execute");
    for core in sys.processors() {
        println!("Core: {}, Frequency: {}mhz, Usage: {:0.2}, Color {}", core.name(), core.frequency(), core.cpu_usage(), get_color(core.cpu_usage()));
    }
    println!("{:?}\n{:?}", output1, output2);
}
