use std::fs::File;
use std::io::prelude::*;

fn read_str(file: &str) -> String {
    let mut file = File::open(file).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.trim().to_string()
}

pub fn description() -> String {
    read_str("/proc/loadavg")
}
