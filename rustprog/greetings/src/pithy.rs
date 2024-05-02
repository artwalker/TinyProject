use rand::Rng;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn print_random_saying(filename: &str) -> io::Result<()> {
    if !Path::new(filename).exists() {
        panic!("File {} does not exist", filename);
    }

    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut list_base = Vec::new();

    for line in reader.lines() {
        let line = line?;
        list_base.push(line);
    }

    if list_base.is_empty() {
        eprintln!("Unable to open file {}", filename);
        std::process::exit(1);
    }

    // Randomly select a saying from list_base and print it
    let saying = rand::thread_rng().gen_range(0..list_base.len());
    println!("{}", list_base[saying]);

    Ok(())
}
