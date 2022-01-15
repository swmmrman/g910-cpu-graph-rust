use std::process::Command;

fn main() {
    const BACKGROUND: &str = "562500";
    const START_COLOR: &str = "210000";
    let output1 = Command::new("g910-led").args(["-a", START_COLOR]).output().expect("Failed to execute");
    let output2 = Command::new("g910-led").args(["-a", BACKGROUND]).output().expect("Failed to execute");

    println!("{:?}\n{:?}", output1, output2);
}
