# bigger_is_greater.rs

Problem Overview:
The goal is to find the next lexicographical permutation of a given string w. If no such permutation exists (i.e., the string is the largest lexicographical permutation), return "no answer".

Function Implementation:
```rust 
fn biggerIsGreater(w: &str) -> String {}
```

This function takes a string slice w as input and returns the next lexicographical permutation as a String.

Step 1: Convert String to Vector of Characters
```rust 
let mut chars: Vec<char> = w.chars().collect();
```

	•	The input string w is converted to a vector of characters (Vec<char>) for easier manipulation of individual characters.
	•	Example: If w = "dkhc", chars = ['d', 'k', 'h', 'c'].

Step 2: Find the Pivot Index (i)
```rust 
let n = chars.len();
let mut i = n as isize - 2; // Start from the second-to-last character
while i >= 0 && chars[i as usize] >= chars[(i + 1) as usize] {
    i -= 1;
}
```

	•	Purpose: Find the first character (from the right) that is smaller than the character immediately after it.
	•	Why? This marks the position where a swap will create a larger permutation.
	•	Start from the second-to-last character (n - 2) and move leftward until we find chars[i] < chars[i + 1].

Example 1: w = "dkhc"
	1.	Start at index 2 (i = 2, chars[2] = 'h').
	    •	Compare chars[2] ('h') and chars[3] ('c'). 'h' >= 'c', so move left.
	2.	Now i = 1, chars[1] = 'k'.
	    •	Compare chars[1] ('k') and chars[2] ('h'). 'k' >= 'h', so move left.
	3.	Now i = 0, chars[0] = 'd'.
	    •	Compare chars[0] ('d') and chars[1] ('k'). 'd' < 'k', so stop. i = 0 is the pivot.

	•	If i < 0: No pivot found, meaning the string is already the largest permutation (e.g., "dcba"). Return "no answer".

Step 3: Find the Smallest Larger Character (j)
```rust 
let mut j = n as isize - 1; // Start from the last character
while chars[j as usize] <= chars[i as usize] {
    j -= 1;
}
```

	•	Purpose: Find the smallest character to the right of i that is larger than chars[i].

Example: w = "dkhc", pivot i = 0, chars[0] = 'd'
	1.	Start at j = 3 (chars[3] = 'c').
	    •	Compare chars[3] ('c') and chars[0] ('d'). 'c' <= 'd' is false. Keep moving left.
	2.	Compare chars[1] ('k') and chars[0] ('d'). 'k' > 'd'. Stop. j = 1.

Step 4: Swap chars[i] and chars[j]
```rust 
chars.swap(i as usize, j as usize);
```

	•	Swap the characters at indices i and j.

Example: chars = ['d', 'k', 'h', 'c'], i = 0, j = 1
	•	Swap chars[0] ('d') with chars[1] ('k').
	•	Result: chars = ['k', 'd', 'h', 'c'].

Step 5: Reverse the Substring to the Right of i
```rust 
chars[(i as usize + 1)..].reverse();
```

	•	Reverse the portion of the array after i (from i + 1 to the end).
	•	Why? This ensures the smallest lexicographical permutation for the rest of the string.

Example: chars = ['k', 'd', 'h', 'c'], i = 0
	•	Substring to reverse: chars[1..] = ['d', 'h', 'c'].
	•	Reverse it: ['c', 'h', 'd'].
	•	Result: chars = ['k', 'c', 'h', 'd'].

Step 6: Convert Back to String
```rust 
chars.iter().collect()
```

	•	Convert the modified Vec<char> back into a String.

Final Result for w = "dkhc"
	•	Output: "hcdk".


Full Flow for Example Input:

Input: 5\nab\nbb\nhefg\ndhck\ndkhc
	1.	"ab"
	    •	Pivot: i = 0 ('a' < 'b').
	    •	Swap: ['a', 'b'] -> ['b', 'a'].
	    •	Output: "ba".

	2.	"bb"
	    •	No pivot found. Output: "no answer".

	3.	"hefg"
	    •	Pivot: i = 2 ('f' < 'g').
	    •	Swap: ['h', 'e', 'f', 'g'] -> ['h', 'e', 'g', 'f'].
	    •	Output: "hegf".

	4.	"dhck"
	    •	Pivot: i = 2 ('c' < 'k').
	    •	Swap: ['d', 'h', 'c', 'k'] -> ['d', 'h', 'k', 'c'].
	    •	Output: "dhkc".

	5.	"dkhc"
	    •	Pivot: i = 0 ('d' < 'k').
	    •	Swap: ['d', 'k', 'h', 'c'] -> ['h', 'c', 'd', 'k'].
	    •	Output: "hcdk".