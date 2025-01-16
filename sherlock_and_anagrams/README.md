# sherlock_and_anagrams.rs

Overview
The program solves the problem of finding the number of unordered anagrammatic pairs of substrings in a given string. An “anagrammatic pair” means two substrings that are rearrangements of the same characters.
The program handles multiple queries (strings) from standard input, processes each string, and outputs the result to a file specified by an environment variable OUTPUT_PATH.

Code Explanation

1. Imports
```rust 
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::env;
```

	•	std::collections::HashMap: Used to store and count occurrences of sorted substrings efficiently.
	•	std::fs::File: For writing the output to a file.
	•	std::io: For handling standard input.
	•	std::env: To retrieve the value of the OUTPUT_PATH environment variable.

2. Helper Function: sort_string
```rust 
fn sort_string(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort_unstable();
    chars.into_iter().collect()
}
```

	•	This function takes a string slice s as input and returns a sorted version of the string.
	    Steps:
	        1.	Convert the string to a vector of characters.
	        2.	Sort the vector of characters using sort_unstable() for efficiency.
	        3.	Convert the sorted vector back into a string.

	•	Sorting allows us to treat two anagrams as identical. For example, “abc” and “cab” both become “abc”.

3. Core Function: sherlockAndAnagrams
```rust
fn sherlockAndAnagrams(s: &str) -> i32 {
    let mut count = 0;
    let mut substrings: HashMap<String, i32> = HashMap::new();

    for length in 1..=s.len() {
        for start in 0..=s.len() - length {
            let substring = &s[start..start + length];
            let sorted_substring = sort_string(substring);
            *substrings.entry(sorted_substring).or_insert(0) += 1;
        }
    }

    for &freq in substrings.values() {
        if freq > 1 {
            count += freq * (freq - 1) / 2;
        }
    }

    count
}
```

	•	Purpose: This function calculates the number of anagrammatic pairs in a string s.
	    Steps:
	        1.	Initialize Variables:
	            count: Tracks the number of anagrammatic pairs.
	            substrings: A HashMap to store sorted substrings and their frequencies.

	        2.	Generate All Substrings:
	            The outer loop (for length in 1..=s.len()) iterates over all possible substring lengths.
	            The inner loop (for start in 0..=s.len() - length) extracts substrings of the current length.
            	Each substring is sorted using sort_string and added to the substrings HashMap. If it already exists in the map, its count is incremented.

	        3.	Count Pairs:
	            After populating the substrings map, iterate over its values (freq) to calculate the number of anagrammatic pairs.
	            If a substring appears freq times, the number of pairs is freq * (freq - 1) / 2 (combination formula for choosing 2 items from freq).

4. Main Function: main
```rust
fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let q = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..q {
        let s = stdin_iterator.next().unwrap().unwrap();

        let result = sherlockAndAnagrams(&s);

        writeln!(&mut fptr, "{}", result).ok();
    }
}
```

	•	Purpose: Handles input, processes queries, and writes output to a file.
	    Steps:
	        1.	Read Input:
	            Standard input (stdin) is locked and read line by line using lines().
	            The first line specifies the number of queries, q.
	            Each subsequent line contains a string s to analyze.
	
            2.	Process Each Query:
	            For each string s, the function sherlockAndAnagrams is called to compute the number of anagrammatic pairs.
	
            3.	Write Output:
	            The results are written to a file specified by the environment variable OUTPUT_PATH.