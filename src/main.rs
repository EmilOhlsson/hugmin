use std::fs::File;
use std::io::prelude::*;
use chrono::prelude::*;

fn read_value(file: &str) -> u32 {
    let mut file = File::open(file).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.trim().parse::<u32>().unwrap()
}

fn ratio(q: u32, v: u32) -> f32 {
    q as f32 / v as f32
}

fn main() {
    // Battery stuff
    let energy_full_design = read_value("/sys/class/power_supply/BAT0/energy_full_design");
    let energy_full = read_value("/sys/class/power_supply/BAT0/energy_full");
    let energy_now = read_value("/sys/class/power_supply/BAT0/energy_now");

    // Time and date
    let date_time: DateTime<Local> = Local::now();

    println!(
        "CAP: {:.2}% CHARGE: {:.2}% -- {}",
        100f32 * ratio(energy_full, energy_full_design),
        100f32 * ratio(energy_now, energy_full),
        date_time.to_string(),
    );
}
