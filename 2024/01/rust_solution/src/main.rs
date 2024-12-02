use std::fs;
use std::env;

fn main() {
    // Retrieve the file path from the command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        return;
    }
    let file_path = &args[1];

    // Read the content of the file
    let content = fs::read_to_string(file_path).expect("Failed to read the file");

    // Parse the content into a vector of tuples
    let data: Vec<(i32, i32)> = content
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 2 {
                Some((parts[0].parse().ok()?, parts[1].parse().ok()?))
            } else {
                None
            }
        })
        .collect();

    // Compute Solution 1
    let mut left: Vec<i32> = data.iter().map(|&(l, _)| l).collect();
    let mut right: Vec<i32> = data.iter().map(|&(_, r)| r).collect();
    left.sort();
    right.sort();

    let diffs: Vec<i32> = left.iter().zip(right.iter()).map(|(l, r)| (l - r).abs()).collect();
    let solution1: i32 = diffs.iter().sum();
    println!("Solution 1: {}", solution1);

    // Compute Solution 2
    use std::collections::HashMap;
    let mut count_map = HashMap::new();
    for &r in &right {
        *count_map.entry(r).or_insert(0) += 1;
    }

    let scores: Vec<i32> = left.iter().map(|&l| l * count_map.get(&l).unwrap_or(&0)).collect();
    let solution2: i32 = scores.iter().sum();
    println!("Solution 2: {}", solution2);
}
