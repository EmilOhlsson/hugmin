use chrono::prelude::*;

mod battery;
mod network;

const BATTERY: char = '\u{01F50B}';
const NETWORK: char = '\u{01F5A7}';
const CLOCK: char = '\u{01F551}';

fn main() {
    // Time and date
    let date_time: DateTime<Local> = Local::now();

    println!(
        "{} {} {} {} {} {}",
        NETWORK,
        network::description(),
        BATTERY,
        battery::description(),
        CLOCK,
        date_time.to_rfc2822(),
    );
}
