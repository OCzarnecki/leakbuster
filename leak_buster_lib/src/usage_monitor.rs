use std::process::Command;
use std::{thread, time};

pub fn launch_with_monitoring(cmd: &str, args: &Vec<String>) {
    let mut command = Command::new(cmd)
        .args(args)
        .spawn()
        .expect("Failed to execute command");

    let hundred_millis = time::Duration::from_millis(100);
    while command.try_wait().unwrap().is_none() {
        println!("x");
        thread::sleep(hundred_millis);
    }
}
