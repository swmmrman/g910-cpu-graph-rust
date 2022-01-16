use sysinfo::{System, SystemExt, ProcessorExt};
use std::process::Command;
use std::{thread, time};
use std::fs::File;
use std::io::Write;


fn get_color(percent: f32) -> String {
    let percent = percent / 100.0;
    let r = (percent * 255.0) as i32;
    let g =  (110.0 - (percent * 110.0)) as i32;
    format!("{:02X}{:02X}00", r, g)
}

fn main() -> std::io::Result<()> {
    let keys = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "q", "w", "e", "r", "t", "y", "u", "i", "o", "p"];
    let mut sys = System::new_all();
    sys.refresh_all();
    // const BACKGROUND: &str = "562500";
    const START_COLOR: &str = "210000";
    let _output1 = Command::new("g910-led").args(["-a", START_COLOR]).output().expect("Failed to execute");
    thread::sleep(time::Duration::from_secs(1));
    let mut command = format!("a {}", START_COLOR);
    let mut file = File::create("/tmp/g910.profile")?;
    let mut counter = 0;
    for core in sys.processors() {
        let newpart = format!("\nk {} {}", keys[counter], get_color(core.cpu_usage()));
        command.push_str(&newpart);
        counter += 1;
    }
    command.push_str("\nc");
    file.write_all(command.as_bytes())?;
    let _output2 = Command::new("g910-led").args(["-p", "/tmp/g910.profile"]).output().expect("Failed to execute");
    Ok(())
}
