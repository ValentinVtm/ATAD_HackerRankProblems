use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn time_in_words(h: i32, m: i32) -> String {
    // Words representation of numbers
    let numbers = vec![
        "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
        "eleven", "twelve", "thirteen", "fourteen", "quarter", "sixteen", "seventeen",
        "eighteen", "nineteen", "twenty", "twenty one", "twenty two", "twenty three",
        "twenty four", "twenty five", "twenty six", "twenty seven", "twenty eight", "twenty nine",
    ];

    match m {
        0 => format!("{} o' clock", numbers[h as usize]), // If it's exactly on the hour
        1 => format!("one minute past {}", numbers[h as usize]), // If 1 minute past
        15 => format!("quarter past {}", numbers[h as usize]), // Special case: quarter past
        30 => format!("half past {}", numbers[h as usize]), // Special case: half past
        45 => format!(
            "quarter to {}",
            numbers[(h % 12 + 1) as usize]
        ), // Special case: quarter to the next hour
        1..=29 => format!(
            "{} minutes past {}",
            numbers[m as usize],
            numbers[h as usize]
        ), // Minutes past the hour
        31..=59 => format!(
            "{} minutes to {}",
            numbers[(60 - m) as usize],
            numbers[(h % 12 + 1) as usize]
        ), // Minutes to the next hour
        _ => unreachable!(), // Catch-all for invalid input
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let h = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    let m = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let result = time_in_words(h, m);

    writeln!(&mut fptr, "{}", result).ok();
}