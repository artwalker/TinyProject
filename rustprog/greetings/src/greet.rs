// Import necessary modules
use chrono::{Datelike, Local, Timelike};
use std::env;

// Import the pithy module
mod pithy;

/// Calculate the moon phase for a given date.
///
/// # Arguments
///
/// * `year` - The year of the date.
/// * `month` - The month of the date.
/// * `day` - The day of the date.
///
/// # Returns
///
/// * `usize` - The index of the moon phase.
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
    // Define the moon phases
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

    // Get the current date and time
    let now = Local::now();
    // Calculate the moon phase for the current date
    let mp = moon_phase(now.year(), now.month(), now.day());

    // Collect command line arguments
    let args: Vec<String> = env::args().collect();
    // If there is a command line argument, print a personalized greeting
    if args.len() > 1 {
        //println!("args[0]: {}", args[0]);
        println!("Greetings, {}!", args[1]);
    } else {
        // Otherwise, print a generic greeting
        println!("Greetings!");
    }
    // Print the current date and time
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
    // Print the current moon phase
    println!("The moon is {}", phase[mp]);

    // Call the function from pithy
    pithy::print_random_saying("pithy.txt").unwrap();
}
