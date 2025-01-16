use std::io::{self, BufRead};
use std::cmp::max;

// Function to multiply a number represented as a vector of digits
fn multiply(num: &mut Vec<u8>, factor: u32) {
    let mut carry = 0;
    for digit in num.iter_mut() {
        let result = (*digit as u32) * factor + carry;
        *digit = (result % 10) as u8;
        carry = result / 10;
    }
    while carry > 0 {
        num.push((carry % 10) as u8);
        carry /= 10;
    }
}

// Function to compute the factorial using manual big integer operations
fn extra_long_factorials(n: u32) {
    let mut result = vec![1]; // Start with 1 as [1]
    
    // Multiply the number iteratively to compute factorial
    for i in 2..=n {
        multiply(&mut result, i);
    }
    
    // Print the result in reverse (as digits are stored in reverse order)
    for digit in result.iter().rev() {
        print!("{}", digit);
    }
    println!();
}

fn main() {
    // Read input from the user
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