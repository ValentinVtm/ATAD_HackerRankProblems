use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::cmp;

fn encryption(s: &str) -> String {
    // Remove spaces from the input string
    let s_cleaned: String = s.chars().filter(|&c| c != ' ').collect();
    let length = s_cleaned.len();

    // Calculate the dimensions of the grid
    let rows = (length as f64).sqrt().floor() as usize;
    let cols = (length as f64).sqrt().ceil() as usize;
    let (rows, cols) = if rows * cols < length { (rows + 1, cols) } else { (rows, cols) };

    // Create the encrypted message
    let mut encrypted = Vec::new();
    for col in 0..cols {
        let mut word = String::new();
        for row in 0..rows {
            let index = row * cols + col;
            if index < length {
                word.push(s_cleaned.chars().nth(index).unwrap());
            }
        }
        encrypted.push(word);
    }

    // Join the encrypted words with spaces
    encrypted.join(" ")
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = encryption(&s);

    writeln!(&mut fptr, "{}", result).ok();
}