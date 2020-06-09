use chrono::prelude::*;
use std::fs::File;
use std::io::prelude::*;

mod network;

const BATTERY: char = '\u{01F50B}';
const NETWORK: char = '\u{01F5A7}';

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
    // Battery stuff, TODO: create warning on low battery
    let energy_full_design = read_value("/sys/class/power_supply/BAT0/energy_full_design");
    let energy_full = read_value("/sys/class/power_supply/BAT0/energy_full");
    let energy_now = read_value("/sys/class/power_supply/BAT0/energy_now");

    // Time and date
    let date_time: DateTime<Local> = Local::now();

    println!(
        "{} {} {} CAP: {:.2}% CHARGE: {:.2}% -- {}",
        NETWORK,
        network::description(),
        BATTERY,
        100f32 * ratio(energy_full, energy_full_design),
        100f32 * ratio(energy_now, energy_full),
        date_time.to_string(),
    );
}
