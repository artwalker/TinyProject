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