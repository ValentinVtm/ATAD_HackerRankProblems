# extra_long_factorials.rs

Code Overview
The code calculates the factorial of a number  n  by representing the large result as a vector of digits. Each multiplication is handled manually, and the result is printed in the end.

Detailed Breakdown
1. Import Necessary Libraries
```rust 
use std::io::{self, BufRead};
use std::cmp::max;
```

	•	std::io::{self, BufRead}: Used for reading input from the user.
	•	std::cmp::max: Used to find the maximum value, although it’s not used in this particular version of the code (can be removed).

2. Function to Multiply a Big Number by an Integer
```rust 
fn multiply(num: &mut Vec<u8>, factor: u32) {
    let mut carry = 0;
    for digit in num.iter_mut() {
        let result = (*digit as u32) * factor + carry;
        *digit = (result % 10) as u8; // Update current digit
        carry = result / 10;         // Carry over remaining digits
    }
    while carry > 0 {
        num.push((carry % 10) as u8);
        carry /= 10;                 // Process remaining carry
    }
}
```
Purpose:
This function performs the multiplication of a number represented as a vector of digits (num) by an integer (factor).

Step-by-step Explanation:
	1.	num is a mutable reference to a vector where each element is a digit of the big number.
	    For example, the number 123 is stored as [3, 2, 1] (digits in reverse order).

	2.	The function uses a carry variable to handle digits that overflow during multiplication.
	    Example:  15 \times 9 = 135 , so the carry would be  13  (digits beyond the first).

	3.	For each digit:
	    Multiply the digit by factor and add the carry.
	    Update the digit in the vector with result % 10 (last digit).
	    Update the carry with result / 10 (remaining digits).

	4.	After processing all digits, if there’s still a carry left, add its digits to the vector.

3. Function to Compute Factorials
```rust 
fn extra_long_factorials(n: u32) {
    let mut result = vec![1]; // Start with 1 as [1]
    
    for i in 2..=n {
        multiply(&mut result, i); // Multiply result by each number from 2 to n
    }
    
    for digit in result.iter().rev() {
        print!("{}", digit); // Print digits in reverse order
    }
    println!(); // Add a newline after the result
}
```

Purpose:
This function computes the factorial of a number  n  by iteratively multiplying all numbers from 2 to  n  with a vector result that holds the factorial value.

Step-by-step Explanation:
	1.	Initialization:
	    result is initialized to [1], representing the number 1.

	2.	Iterative Multiplication:
	    A for loop iterates through numbers from 2 to  n .
	    Each number is multiplied with result using the multiply function.
	    
        Example:
	        After multiplying by 2, result becomes [2] (2! = 2).
	        After multiplying by 3, result becomes [6] (3! = 6).
	        After multiplying by 4, result becomes [4, 2] (4! = 24, stored as [4, 2]).
	
    3.	Output:
	    The digits in result are stored in reverse order, so they are printed in reverse using result.iter().rev().
	    Example: For 4! = 24, the vector is [4, 2]. Printing in reverse gives 24.

4. Main Function
```rust 
fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();
    
    let n: u32 = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    extra_long_factorials(n);
}
```

Purpose:
The main function reads an integer  n  from the user, then calls extra_long_factorials to compute and print  n! .

Step-by-step Explanation:
	1.	Input Handling:
	    stdin reads the input.
	    stdin.lock().lines() returns an iterator over input lines.
	    The first line is read, trimmed of whitespace, and parsed into an integer n.

	2.	Calling the Factorial Function:
	    The extra_long_factorials function is called with  n .
	
    3.	Example Input/Output:
	    Input: 5
	    Execution:
	        Compute  5! = 1 \times 2 \times 3 \times 4 \times 5 = 120 .
	    Output: 120