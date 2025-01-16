# larry_s_array.rs

Step 1: Problem Breakdown

The task is to determine if a given array A can be sorted using a specific 3-element rotation operation, repeated any number of times. The key insight here is to use the concept of inversions:
	•	An inversion is a pair of indices (i, j) such that:

A[i] > A[j] and i < j

	•	If the total number of inversions is even, the array can be sorted using the rotation operation. Otherwise, it cannot.

Step 2: Input Handling
The main function reads the input values and processes each test case.

let stdin = io::stdin();
let mut stdin_iterator = stdin.lock().lines();

```rust 
let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
```

	1.	t is the number of test cases.
	2.	The input for each test case contains:
	    •	n: The size of the array A.
	    •	A: The array itself.

The program loops through each test case to process the data:

```rust 
for _ in 0..t {
    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let A: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = larrysArray(&A);

    writeln!(&mut fptr, "{}", result).ok();
}
```

	•	For each test case, the array A is read and passed to the larrysArray function.
	•	The result ("YES" or "NO") is written to the output file.

Step 3: Counting Inversions
The larrysArray function is where the key logic is implemented.

```rust 
fn larrysArray(A: &[i32]) -> String {
    let mut inversions = 0;

    for i in 0..A.len() {
        for j in i + 1..A.len() {
            if A[i] > A[j] {
                inversions += 1;
            }
        }
    }}
```

	•	Outer Loop: Iterates through the array index i (from 0 to A.len() - 1).
	•	Inner Loop: For each i, compares A[i] with all elements to its right (j > i).
	•	Condition A[i] > A[j]: If A[i] is greater than A[j], it counts as an inversion, so the inversions counter is incremented.

Step 4: Check Parity of Inversions
After counting inversions, we check whether the total number of inversions is even or odd:

```rust 
if inversions % 2 == 0 {
    "YES".to_string()
} else {
    "NO".to_string()
}
```

	•	If the number of inversions is even, the array can be sorted using the 3-element rotation, and the function returns "YES".
	•	Otherwise, it returns "NO".

Step 5: Output Results
The main function writes the result of each test case to the output file:

```rust 
writeln!(&mut fptr, "{}", result).ok();
```