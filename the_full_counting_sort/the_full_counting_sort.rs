use std::io::{self, BufRead};

/*
 * Complete the 'countSort' function below.
 *
 * The function accepts 2D_STRING_ARRAY arr as parameter.
 */

fn countSort(arr: &[Vec<String>]) {
    // Determine the size of the auxiliary array based on the largest integer in the input
    let max_index = arr.iter().map(|entry| entry[0].parse::<usize>().unwrap()).max().unwrap();
    
    // Create an auxiliary array with empty vectors to store strings for each integer key
    let mut buckets: Vec<Vec<String>> = vec![vec![]; max_index + 1];

    // Iterate through the input array and distribute the strings into the appropriate buckets
    for (i, entry) in arr.iter().enumerate() {
        let key = entry[0].parse::<usize>().unwrap();
        if i < arr.len() / 2 {
            // Replace the first half of strings with "-"
            buckets[key].push("-".to_string());
        } else {
            // Push the actual strings for the second half
            buckets[key].push(entry[1].clone());
        }
    }

    // Flatten the buckets into a single sorted array and print the result
    let result: String = buckets.into_iter().flat_map(|bucket| bucket).collect::<Vec<_>>().join(" ");
    println!("{}", result);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // Read the number of input lines
    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut arr: Vec<Vec<String>> = Vec::with_capacity(n as usize);

    // Read the input and parse it into the `arr` 2D vector
    for _ in 0..n as usize {
        let line = stdin_iterator.next().unwrap().unwrap();
        let split: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
        arr.push(split);
    }

    // Call the countSort function
    countSort(&arr);
}