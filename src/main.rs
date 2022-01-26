use sysinfo::{System, SystemExt, ProcessorExt};
use std::process::Command;
use std::{thread, time, fs};
use std::fs::File;
use std::io::Write;


fn get_color(percent: f32) -> String {
    let percent = percent / 100.0;
    let r = (percent * 255.0) as i32;
    let g =  (110.0 - (percent * 110.0)) as i32;
    format!("{:02X}{:02X}00", r, g)
}

fn get_cpu_temp(path: &str) -> f32 {
    let temp: String = fs::read_to_string(path)
        .expect(&format!("File missing {}", path));
    let temp = temp.trim().parse::<f32>().unwrap();
    temp / 1000.0
}

fn main() -> std::io::Result<()> {
    let mut cpu_file = String::new();
    let thermals = fs::read_dir("/sys/class/thermal/")?;
    for thermal in thermals {
        let type_path = format!("{}/type", &thermal.as_ref().unwrap().path().display());
        let sensor_type = fs::read_to_string(type_path).expect("Oops");
        if sensor_type == "x86_pkg_temp\n" {
            cpu_file = format!("{}/temp", &thermal.unwrap().path().display());
        }
    }
    let running = true;
    let keys = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "q", "w", "e", "r", "t", "y", "u", "i", "o", "p"];
    let mut sys = System::new_all();
    // const BACKGROUND: &str = "562500";
    const START_COLOR: &str = "210000";
    let _output1 = Command::new("g910-led").args(["-a", START_COLOR]).output().expect("Failed to execute");
    thread::sleep(time::Duration::from_secs(1));
    while running {
        sys.refresh_all();
        let mut command = format!("a {}", START_COLOR);
        let mut file = File::create("/tmp/g910.profile")?;
        let mut counter = 0;
        for core in sys.processors() {
            let newpart = format!("\nk {} {}", keys[counter], get_color(core.cpu_usage()));
            command.push_str(&newpart);
            counter += 1;
        }
        let cpu_temp = get_cpu_temp(&cpu_file);
        let temp_str = format!("\ng logo {}", get_color((cpu_temp-30.0)*1.4));
        command.push_str(&temp_str);
        command.push_str("\nc");
        file.write_all(command.as_bytes())?;
        let _output2 = Command::new("g910-led").args(["-p", "/tmp/g910.profile"]).output().expect("Failed to execute");
        thread::sleep(time::Duration::from_millis(500));
    }
    Ok(())
}
