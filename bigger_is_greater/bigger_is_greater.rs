use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'biggerIsGreater' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING w as parameter.
 */

fn biggerIsGreater(w: &str) -> String {
    // Convert the input string to a vector of characters for easy manipulation
    let mut chars: Vec<char> = w.chars().collect();
    let n = chars.len();

    // Step 1: Find the first character from the right that is smaller than the one after it
    let mut i = n as isize - 2; // Start at the second last character
    while i >= 0 && chars[i as usize] >= chars[(i + 1) as usize] {
        i -= 1;
    }

    // If no such character exists, return "no answer"
    if i < 0 {
        return "no answer".to_string();
    }

    // Step 2: Find the smallest character to the right of `i` that is larger than `chars[i]`
    let mut j = n as isize - 1;
    while chars[j as usize] <= chars[i as usize] {
        j -= 1;
    }

    // Step 3: Swap the characters at `i` and `j`
    chars.swap(i as usize, j as usize);

    // Step 4: Reverse the substring to the right of `i`
    chars[(i as usize + 1)..].reverse();

    // Convert the vector of characters back to a string and return it
    chars.iter().collect()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let T = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..T {
        let w = stdin_iterator.next().unwrap().unwrap();

        let result = biggerIsGreater(&w);

        writeln!(&mut fptr, "{}", result).ok();
    }
}