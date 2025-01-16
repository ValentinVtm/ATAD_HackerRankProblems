use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'pairs' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER k
 *  2. INTEGER_ARRAY arr
 */

fn pairs(k: i32, arr: &[i32]) -> i32 {
    // Use a HashSet to store array elements for O(1) lookups
    let mut set: HashSet<i32> = HashSet::new();
    let mut count = 0;

    // Add all elements to the HashSet
    for &num in arr {
        set.insert(num);
    }

    // Check for pairs where `num - k` exists in the set
    for &num in arr {
        if set.contains(&(num - k)) {
            count += 1;
        }
    }

    count
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let n = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let k = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = pairs(k, &arr);

    writeln!(&mut fptr, "{}", result).ok();
}