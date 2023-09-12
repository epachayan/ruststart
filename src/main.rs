use std::io;
use rayon::prelude::*;

fn main() {
    println!("Hello, you are in Parallel !");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    println!("You typed: {}", input);

    //let mut list = vec![1, 5, 3, 2, 4];
    let mut list = parse_input(&input);

    parallel_quicksort(&mut list);

    for i in &list {
        println!("{}", i);
    }
}

fn parallel_quicksort(list: &mut Vec<i32>) {
    // Check if the list is small enough to use single-threaded quicksort
    if list.len() <= threshold {
        list.sort();
        return;
    }

    // Use Rayon's parallel sorting
    list.par_sort();
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .split_whitespace() // Split input by whitespace
        .filter_map(|s| s.parse().ok()) // Parse each part as i32
        .collect() // Collect the parsed numbers into a Vec<i32>
}

// Define a threshold value for when to use single-threaded quicksort
const threshold: usize = 5; // Adjust as needed
