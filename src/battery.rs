use std::fs::File;
use std::io::prelude::*;

fn read_value(file: &str) -> u32 {
    let mut file = File::open(file).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.trim().parse::<u32>().unwrap()
}

fn ratio(q: u32, v: u32) -> f32 {
    q as f32 / v as f32
}

pub fn description() -> String {
    // Battery stuff, TODO: create warning on low battery
    let energy_full_design = read_value("/sys/class/power_supply/BAT0/energy_full_design");
    let energy_full = read_value("/sys/class/power_supply/BAT0/energy_full");
    let energy_now = read_value("/sys/class/power_supply/BAT0/energy_now");

    format!(
        "{:.2}% / {:.2}%",
        100f32 * ratio(energy_now, energy_full),
        100f32 * ratio(energy_full, energy_full_design),
    )
}
