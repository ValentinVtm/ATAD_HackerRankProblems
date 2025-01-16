# pairs.rs

Overview
The code calculates the number of pairs in an array whose difference equals a given target value  k , using a HashSet for efficient lookup to achieve linear time complexity.

1. Function Signature
```rust 
fn pairs(k: i32, arr: &[i32]) -> i32
```

	•	Input Parameters:
	        k: The target difference between two numbers in the array.
	        arr: A slice of integers representing the input array.

	•	Output:
	        Returns the number of pairs (x, y) in arr where x - y = k.

2. HashSet for Fast Lookups
```rust 
let mut set: HashSet<i32> = HashSet::new();
```

	•	A HashSet is a data structure that allows quick insertion and lookup (average O(1) time complexity).
	•	We’ll use the set to store all the elements of the array so that we can check if a specific number exists in constant time.

3. Add All Array Elements to HashSet
```rust 
for &num in arr {
    set.insert(num);
}
```

	•	Purpose: Add every element in arr to the set.
	•	Reason: This allows us to check if a value (like num - k) exists in the array using the set.contains() method.
	•	For the example input arr = [1, 5, 3, 4, 2], the set will contain: {1, 5, 3, 4, 2}.

4. Iterate Through Array to Find Pairs
```rust 
for &num in arr {
    if set.contains(&(num - k)) {
        count += 1;
    }
}
```

	•	We loop through each number in the array (num).
	•	For each num:
	        Calculate the target value: num - k.
	        Check if num - k exists in the set.
	        If it does, increment the count because it means there’s a valid pair (num, num - k).

5. Return the Count
```rust 
return count;
```

6. Main Function Setup

Input Handling:
```rust 
let stdin = io::stdin();
let mut stdin_iterator = stdin.lock().lines();

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
```

	1.	Input Reading:
	    Read multiple lines of input using stdin.lock().lines().
	    Parse the first line to extract n (number of elements in the array) and k (the target difference).
	    Parse the second line to create a vector arr containing the integers in the array.

	2.	Parsing Explanation:
	    first_multiple_input splits the first input line into a vector of strings and parses them as integers.
	    arr is parsed by splitting the second line of input and converting each string into an integer.


Calling the Function:
```rust 
let result = pairs(k, &arr);
```

	•	Call the pairs function with k and arr as arguments.
	•	Store the returned result in the result variable.

Writing Output to File:
```rust 
writeln!(&mut fptr, "{}", result).ok();
```

	•	Write the output (result) to a file whose path is specified by the OUTPUT_PATH environment variable.