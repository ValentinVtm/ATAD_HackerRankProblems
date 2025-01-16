fn surfaceArea(A: &[Vec<i32>]) -> i32 {
    let h = A.len();
    let w = A[0].len();
    let mut surface_area = 0;

    // Directions for neighbors: up, down, left, right
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for i in 0..h {
        for j in 0..w {
            let height = A[i][j];

            // Add top and bottom areas
            surface_area += 2;

            // Check each neighbor
            for &(dx, dy) in &directions {
                let ni = (i as isize + dx) as usize;
                let nj = (j as isize + dy) as usize;

                let neighbor_height = if ni < h && nj < w { A[ni][nj] } else { 0 };
                surface_area += (height - neighbor_height).max(0);
            }
        }
    }

    surface_area
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let first_line = stdin_iterator.next().unwrap().unwrap();
    let first_multiple_input: Vec<i32> = first_line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let H = first_multiple_input[0];
    let W = first_multiple_input[1];

    let mut A: Vec<Vec<i32>> = Vec::new();

    for _ in 0..H {
        let line = stdin_iterator.next().unwrap().unwrap();
        let row: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        A.push(row);
    }

    let result = surfaceArea(&A);
    println!("{}", result);
}