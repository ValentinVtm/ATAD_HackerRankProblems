# 3D_surface_area.rs

What the Code Does

The goal of this code is to calculate the 3D surface area of a toy made by stacking cubes based on a given 2D array of heights (A). Each element in the 2D array represents the height of cube stacks at that position.

The surface area is calculated by:
	1.	Adding the top and bottom areas of the cubes.
	2.	Adding the visible side areas based on the differences in height between neighboring cells (up, down, left, and right).


How the Code Works

1. Function: ```rust surfaceArea(A: &[Vec<i32>]) -> i32 ```

This function performs the main computation of the surface area. Below is the breakdown:

Step 1: Initialize Variables
```rust
let h = A.len();
let w = A[0].len();
let mut surface_area = 0;
```

	•	h and w are the dimensions (height and width) of the 2D array A.
	•	surface_area is initialized to 0 to store the total surface area.

Step 2: Define Neighbor Directions
```rust
let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
```

	•	These represent the relative positions of the four neighbors:
	        (-1, 0) → above.
	        (1, 0) → below.
	        (0, -1) → left.
	        (0, 1) → right.

Step 3: Iterate Through Each Cell
```rust
for i in 0..h {
    for j in 0..w {
        let height = A[i][j];}}
```

	•	Loops through every position (i, j) in the 2D array A.
	•	The height of the stack at position (i, j) is stored in height.

Step 4: Add Top and Bottom Areas
```rust
surface_area += 2;
```

	•	Each stack contributes a top and bottom area of 1 unit per face, for a total of 2.

Step 5: Compare Height With Neighbors
```rust
for &(dx, dy) in &directions {
    let ni = (i as isize + dx) as usize;
    let nj = (j as isize + dy) as usize;

    let neighbor_height = if ni < h && nj < w { A[ni][nj] } else { 0 };
    surface_area += (height - neighbor_height).max(0);
}
```

	•	For each of the four neighbors:
	        Calculate the neighbor’s position (ni, nj) by adding the direction offsets (dx, dy) to the current position (i, j).
	        Check if the neighbor is within bounds:
	            If it’s within bounds, get its height from the array A.
	            If it’s out of bounds (e.g., edge or corner), treat the neighbor’s height as 0 (air).
	    Add the difference in height between the current stack and its neighbor to the surface area, but only if the current stack is taller ((height - neighbor_height).max(0) ensures non-negative contribution).


2. Main Function

The main function handles input and output.

Step 1: Read Input
```rust
let first_line = stdin_iterator.next().unwrap().unwrap();
let first_multiple_input: Vec<i32> = first_line.split_whitespace()
    .map(|s| s.parse().unwrap())
    .collect();

let H = first_multiple_input[0];
let W = first_multiple_input[1];
```

	•	Reads the first line of input, which contains the dimensions of the grid H and W.

Step 2: Parse the Grid
```rust
let mut A: Vec<Vec<i32>> = Vec::new();

for _ in 0..H {
    let line = stdin_iterator.next().unwrap().unwrap();
    let row: Vec<i32> = line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    A.push(row);
}
```

	•	Reads the next H lines, where each line represents a row of the grid A. Each value in a row represents the height of cubes at that position.

Step 3: Compute Surface Area
```rust
let result = surfaceArea(&A);
println!("{}", result);
```

	•	Calls the surfaceArea function with the grid A as input.
	•	Prints the computed surface area.