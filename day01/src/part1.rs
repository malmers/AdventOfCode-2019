use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let filename = "input";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut stack = Vec::new();
    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for line in reader.lines() {
        let number: i32 = line?.parse().unwrap(); // Ignore errors.
        // Show the line and its number.
        stack.push(number);
    }

    let sum = stack.iter()
        .map(|x| x / 3 - 2)
        .fold(0, |acc, x| acc + x);

    println!("{}", sum);

    Ok(())
}
