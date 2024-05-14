# Using and Understanding the Greetings Rust Program

This project is based on the source code from the book "Tiny C Projects". The code for this project can be found on my GitHub at Greetings.
> Site: https://github.com/artwalker/TinyProject/tree/main/rustprog/greetings  

There are also implementations of this project in other programming languages.

## Project Analysis
This Rust project is a command-line application that prints a greeting, the current date and time, the moon phase for the current date, and a random saying from a file named "pithy.txt".

The moon_phase function calculates the moon phase based on the given date. It uses a specific algorithm to determine the moon phase index.

The main function defines the moon phases, gets the current date and time, and calculates the moon phase for the current date. It collects command-line arguments and prints a personalized greeting if an argument is provided. It then prints the current date, time, and moon phase. Finally, it calls pithy::print_random_saying to print a random saying from "pithy.txt".

This Rust build script is used to manage symbolic links and copy files during the build process.

It first determines the paths for the source and destination of two files: an executable named "greetings" and a text file named "pithy.txt". The source paths are derived from the build directory and the project directory, while the destination paths are in a "bin" directory in the user's home directory.

If the CARGO_CLEAN environment variable is set to "1", indicating a clean operation, the script removes the symbolic links at the destination paths.

Otherwise, it first attempts to remove any existing symbolic links at the destination paths. Then, it creates new symbolic links from the source paths to the destination paths. If any of these operations fail, an error message is printed.

Finally, it copies the "pithy.txt" file to a directory three levels up from the build directory. If the copy operation fails, the script panics and prints an error message.

## Usage Guide

Compile the project: In the terminal, navigate to the directory containing the Makefile, then enter make or make all. This will compile the source files, link the generated object files to create an executable file.

1. Compile the project: In the terminal, navigate to the directory containing the `greetings`, then enter `cargo build --release`. This will compile the source files, link the generated object files to create an executable file.
2. Run the program: In the terminal, navigate to the ~/bin directory, then enter ./greetings. This will run the program, and the program will randomly select and print a quote from the pithy.txt file. Or
    ```bash
    cd target/release
    ./greetings Alice
    ```
3. Clean up the project: If you want to delete the generated executable file and object files, as well as the links created in the ~/bin directory, you can enter `cargo clean` in the terminal.

4. Configure .zshrc: If you want to automatically run the greetings program when starting a new shell session, you can add the following line to the .zshrc file:


## Code Analysis
### Tree
```bash
greetings
├── Cargo.lock
├── Cargo.toml
├── README.md
├── build.rs
└── src
    ├── greet.rs
    ├── pithy.rs
    └── pithy.txt
```
### greet.rs
```rs
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
```
### pithy.rs
```rs
use rand::Rng; // Import the random number generator
use std::fs::File; // Import the file handling module
use std::io::{self, BufRead}; // Import the IO and buffered reading modules
use std::path::Path; // Import the path handling module

/// Prints a random saying from a given file.
///
/// # Arguments
///
/// * `filename` - A string slice that holds the name of the file
///
/// # Panics
///
/// Panics if the file does not exist or cannot be read.
///
/// # Returns
///
/// Returns an `io::Result<()>`. If the operation is successful, the function will return `Ok(())`.
/// If the operation fails, the function will return `Err` with the cause of the failure.
pub fn print_random_saying(filename: &str) -> io::Result<()> {
    // Check if the file exists
    if !Path::new(filename).exists() {
        panic!("File {} does not exist", filename); // If the file does not exist, panic
    }

    let path = Path::new(filename); // Create a path object
    let file = File::open(&path)?; // Open the file
    let reader = io::BufReader::new(file); // Create a buffered reader

    let mut list_base = Vec::new(); // Create an empty vector to store each line from the file

    // Iterate over each line in the file
    for line in reader.lines() {
        let line = line?; // Read the line
        list_base.push(line); // Add the line to the vector
    }

    // If the vector is empty, i.e., the file has no content, print an error message and exit the program
    if list_base.is_empty() {
        eprintln!("Unable to open file {}", filename);
        std::process::exit(1);
    }

    // Randomly select a saying from list_base and print it
    let saying = rand::thread_rng().gen_range(0..list_base.len()); // Generate a random number
    println!("{}", list_base[saying]); // Print the randomly selected line

    Ok(()) // Return Ok
}
```
### Cargo.toml
```toml
[package]
# The name of the package
name = "greetings"
# The version of the package
version = "0.1.0"
# The authors of the package
authors = ["Artwlker <xinxingwang@acm.org>"]
# The edition of Rust to use
edition = "2021"
# The build script to run before compiling the package
build = "build.rs"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
# The chrono crate for working with dates and times
chrono = "0.4"
# The rand crate for generating random numbers
rand = "0.8"

[[bin]]
# The name of the binary to produce
name = "greetings"
# The path to the source file for the binary
path = "src/greet.rs"
```

### pithy.txt
```txt
Politics exists so that uncoordinated people can play sports.
Water alone doesn't get you clean. You must use soap. That's because dirt and crud loves soap and sticks to it really well. The water then washes away the soap, along with the dirt, and the result is that you are clean.
You buy popcorn, soda, and candy so that you have something to eat before the movie starts.
Just wait until Starbucks figures out that you can snort coffee.
Nothing instills more doubt in the curious than the sign "Wet Paint."
You might dislike texting, but it certainly does get annoying people to shut up.
You know you have an eating problem when you finish a meal and think, "Boy! When can I do that again?"
The middle of nowhere is equidistant from everywhere else.
Marketing wizards are looking for the human equivalent of what a dog feels at the sound of a can opener.
Having a pet ensures that you don't freak out over every noise in the house. Loud bang? It's the cat. So what if the cat is in the room with me. It's the cat.
The true experience at an amusement park is waiting in lines.
There is no logic in the computer industry.
The car's manual calls it the "check engine" lamp, but I call it the "This is going to cost at least $200" light.
You drive on a parkway and park on a driveway.
Do I take a break from work to play a video game, or take a break from a video game to get work done?
```