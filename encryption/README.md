# encryption.rs

1. Function Overview
The encryption function takes a string s as input and returns the encrypted string following the steps of the encryption problem.

2. Remove Spaces
```rust 
let s_cleaned: String = s.chars().filter(|&c| c != ' ').collect();
```

	•	What it does:
    It removes all spaces from the input string s by iterating over its characters (chars()), filtering out spaces (c != ' '), and collecting the remaining characters into a new string s_cleaned.

	•	Why:
	The problem specifies that spaces should be ignored during the encryption process.

3. Calculate the Length
```rust 
let length = s_cleaned.len();
```

	•	What it does:
	Determines the length of the cleaned string (without spaces).

	•	Why:
	The length is needed to calculate the grid dimensions for the encryption process.

4. Compute Grid Dimensions
```rust 
let rows = (length as f64).sqrt().floor() as usize;
let cols = (length as f64).sqrt().ceil() as usize;
let (rows, cols) = if rows * cols < length { (rows + 1, cols) } else { (rows, cols) };
```

	•	What it does:
	Calculates the number of rows and columns for the grid:
	    rows: The largest integer less than or equal to the square root of the string length (floor(sqrt(L))).
	    cols: The smallest integer greater than or equal to the square root of the string length (ceil(sqrt(L))).
	Ensures the grid dimensions satisfy rows * cols >= length:
	    If the current rows * cols is too small to fit the string, it increases the number of rows by 1.

	•	Why:
	The problem requires the grid dimensions to meet the constraints:
	    rows <= cols
	    rows * cols >= length

5. Build the Encrypted Message
```rust 
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
```

	•	What it does:
	Iterates over each column (col) of the grid.
	For each column:
	    Iterates through all rows (row) in that column.
	    Calculates the index in the cleaned string for the current character:
	        index = row * cols + col
	    Adds the character at the calculated index to the current word if the index is within bounds.
	Appends each column’s word to the encrypted vector.

	•	Why:
	This constructs the column-wise words of the grid, which form the encrypted message.

6. Combine the Words
```rust
encrypted.join(" ")
```

	•	What it does:
	Joins all the words in the encrypted vector with spaces between them.

	•	Why:
	The problem specifies that the output should consist of the words separated by spaces.

7. Main Function
```rust
let stdin = io::stdin();
let mut stdin_iterator = stdin.lock().lines();
let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();
let s = stdin_iterator.next().unwrap().unwrap();
let result = encryption(&s);
writeln!(&mut fptr, "{}", result).ok();
```

	•	What it does:
	Reads the input string from standard input (stdin).
	Calls the encryption function to encrypt the string.
	Writes the result to a file specified by the environment variable OUTPUT_PATH.

	•	Why:
	This is the entry point of the program, where it connects the input/output and processes the encryption.