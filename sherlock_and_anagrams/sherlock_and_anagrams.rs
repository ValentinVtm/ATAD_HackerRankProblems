use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::env;

// Sort the characters in a string
fn sort_string(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort_unstable();
    chars.into_iter().collect()
}

// Function to count anagrammatic pairs
fn sherlockAndAnagrams(s: &str) -> i32 {
    let mut count = 0;
    let mut substrings: HashMap<String, i32> = HashMap::new();

    // Generate all substrings and store their sorted form in the HashMap
    for length in 1..=s.len() {
        for start in 0..=s.len() - length {
            let substring = &s[start..start + length];
            let sorted_substring = sort_string(substring);
            *substrings.entry(sorted_substring).or_insert(0) += 1;
        }
    }

    // Count pairs of anagrams
    for &freq in substrings.values() {
        if freq > 1 {
            count += freq * (freq - 1) / 2;
        }
    }

    count
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // Create output file
    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    // Read number of queries
    let q = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..q {
        let s = stdin_iterator.next().unwrap().unwrap();

        let result = sherlockAndAnagrams(&s);

        writeln!(&mut fptr, "{}", result).ok();
    }
}