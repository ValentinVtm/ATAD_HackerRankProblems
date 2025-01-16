# the_full_counting_sort.rs

This Rust code implements a stable counting sort algorithm to sort an array of strings associated with integers. It adheres to specific problem constraints: maintaining the stability of the sort and replacing strings in the first half of the input with "-".

Step-by-Step Explanation

1. Input Parsing (in main)
	•	Input format:
	        The first line contains n, the number of integer/string pairs.
	        The next n lines contain two values: an integer key (x) and a string (s).

	•	The program reads these lines and stores them in a 2D vector arr, where:
	        arr[i][0] is the integer key as a string.
	        arr[i][1] is the associated string.

	•	How it’s done:
	        stdin_iterator is used to read lines from standard input.
	        Each line is split by spaces and mapped into a Vec<String> to form the array.

2. Setting Up Buckets in countSort
	•	Counting sort relies on “buckets,” which group strings by their integer keys.
	•	The maximum integer key is determined to allocate an auxiliary array of buckets (buckets), where each bucket is a Vec<String>.

3. Distributing Strings into Buckets
	•	The input array arr is iterated.

	•	Each entry’s key determines the bucket it belongs to:
	        If the entry belongs to the first half (i < arr.len() / 2), the associated string is replaced by "-".
	        Otherwise, the actual string is added to the bucket.

	•	Why this step works:
	        The program ensures that all strings with the same integer key are placed in the correct bucket while preserving their original order (stability).

4. Flattening the Buckets
	•	Once all strings are distributed, the buckets are flattened into a single sorted list using:
```rust
buckets.into_iter().flat_map(|bucket| bucket)
```

	•	Each bucket’s contents are concatenated, ensuring that strings with smaller integer keys appear earlier in the result.

5. Outputting the Result
	•	The flattened list is converted into a single space-separated string using .join(" ") and printed.