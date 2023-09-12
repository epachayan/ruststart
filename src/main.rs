use std::io;

fn main() {
    println!("Hello, world!");
    let mut input = String::new();  
    io::stdin().read_line(&mut input).expect("Failed to read input");

    println!("You typed: {}", input);

    //let mut list = vec![1, 5, 3, 2, 4];
    let mut list = parse_input(&input);
    sort_list(&mut list);

    for i in &list {
        println!("{}", i);
    }   
}

// Create a function to sort the list of numbers using quicksort
fn sort_list(list: &mut Vec<i32>) {
    quicksort(list, 0, list.len() - 1);
}

// Create a function to sort the list of numbers using quicksort
fn quicksort(list: &mut Vec<i32>, low: usize, high: usize) {
    if low >= high {
        return;
    }

    let p = partition(list, low, high);
    if p > 0 {
        quicksort(list, low, p - 1);
    }
    quicksort(list, p + 1, high);
}

// Create a function to partition the list for quicksort
fn partition(list: &mut Vec<i32>, low: usize, high: usize) -> usize {
    let pivot = list[high];
    let mut i = low;

    for j in low..high {
        if list[j] < pivot {
            list.swap(i, j);
            i += 1;
        }
    }

    list.swap(i, high);
    i
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .split_whitespace() // Split input by whitespace
        .filter_map(|s| s.parse().ok()) // Parse each part as i32
        .collect() // Collect the parsed numbers into a Vec<i32>
}

