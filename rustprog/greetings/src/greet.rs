use chrono::{Datelike, Local, Timelike};
use std::env;
mod pithy;

fn moon_phase(year: i32, month: u32, day: u32) -> usize {
    let mut d = day;
    if month == 2 {
        d += 31;
    } else if month > 2 {
        d += 59 + ((month - 3) as f64 * 30.6 + 0.5) as u32;
    }
    let g = (year - 1900) % 19;
    let mut e = (11 * g + 29) % 30;
    if e == 25 || e == 24 {
        e += 1;
    }
    ((((e + d as i32) * 6 + 5) % 177) / 22 & 7) as usize
}

fn main() {
    let phase = [
        "waxing crescent",
        "at first quarter",
        "waxing gibbous",
        "full",
        "waning gibbous",
        "at last quarter",
        "waning crescent",
        "new",
    ];

    let now = Local::now();
    let mp = moon_phase(now.year(), now.month(), now.day());

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("Greetings, {}!", args[1]);
    } else {
        println!("Greetings!");
    }
    println!(
        "Today is {}, {}, {}, {}\nIt is {}:{}:{}",
        now.weekday(),
        now.month(),
        now.day(),
        now.year(),
        now.hour(),
        now.minute(),
        now.second()
    );
    println!("The moon is {}", phase[mp]);

    // Call the function from pithy
    pithy::print_random_saying("pithy.txt").unwrap();
}
