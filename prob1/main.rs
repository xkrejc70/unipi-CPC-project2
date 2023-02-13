mod lib;

use std::io::{self, BufRead};

fn main() {
    let mut arr: Vec<usize> = Vec::new();
    let mut qrs: Vec<Vec<usize>> = Vec::new();

    // Parse and validate input
    (arr, qrs) = parse_input();

    let arr_len = arr.len() - 1;

    // Create segment tree
    let mut seg_tree: Vec<lib::Node> = lib::create_segment_tree(arr);

    // Execute queries
    lib::execute_query(&mut seg_tree, qrs, arr_len);
}

fn parse_input() -> (Vec<usize>, Vec<Vec<usize>>) {
    let mut input_lines: io::Lines<io::StdinLock> = io::stdin().lock().lines();
    let mut line_num: usize = 1;
    let mut query_num: usize = 0;
    let mut n: usize = 0;
    let mut m: usize = 0;
    let mut arr: Vec<usize> = Vec::new();
    let mut qrs: Vec<Vec<usize>> = Vec::new();

    while let Some(line) = input_lines.next() {
        let line = line.unwrap();

        if line.is_empty() {
            break;
        }

        match line_num {
            // 1st line (n, m)
            1 => (n, m) = parse_nums(&line),
            // 2nd line (array A)
            2 => arr = parse_array(n, &line),
            // Queries
            _ => {
                qrs.push(parse_query(&line));
                query_num += 1;
            }
        }

        if query_num > m {
            invalid_input("number of queries");
        }

        line_num += 1;
    }

    if query_num < m {
        invalid_input("number of queries");
    }

    (arr, qrs)
}

// Parse first line (n, m)
fn parse_nums(str: &str) -> (usize, usize) {
    let nums: Vec<String> = split_by_space(str);

    if nums.len() != 2 {
        invalid_input("first line length");
    }

    // Check number format
    if !is_valid_number(&nums[0]) || !is_valid_number(&nums[1]) {
        invalid_input("number format");
    }

    let n: usize = nums[0].parse().unwrap();
    let m: usize = nums[1].parse().unwrap();

    (n, m)
}

// Parse second line (array A)
fn parse_array(n: usize, a: &str) -> Vec<usize> {
    let array: Vec<String> = split_by_space(a);
    let mut array_res: Vec<usize> = Vec::new();

    if array.len() != n {
        invalid_input("array length");
    }

    for elem in array.iter() {
        if !is_valid_number(elem) {
            invalid_input("not a number");
        }
        array_res.push(elem.parse::<usize>().unwrap());
    }

    array_res
}

// Parse query line
fn parse_query(query: &str) -> Vec<usize> {
    let query: Vec<String> = split_by_space(query);
    let mut query_res: Vec<usize> = Vec::new();

    if query.len() < 3 {
        invalid_input("query length");
    }

    for elem in query.iter() {
        if !is_valid_number(elem) {
            invalid_input("not a number");
        }
        query_res.push(elem.parse::<usize>().unwrap());
    }

    match query[0].parse::<usize>().unwrap() {
        0 => check_update(query),
        1 => check_max(query),
        _ => invalid_input("query"),
    }

    query_res
}

fn check_update(query: Vec<String>) {
    if query.len() != 4 {
        invalid_input("update query length");
    }

    check_validity(&query);
}

fn check_max(query: Vec<String>) {
    if query.len() != 3 {
        invalid_input("max query length");
    }

    check_validity(&query);
}

fn check_validity(query: &[String]) {
    for elem in query.iter() {
        if !is_valid_number(elem) {
            invalid_input("not a number");
        }
    }
}

fn split_by_space(str: &str) -> Vec<String> {
    return str.split(' ').map(|s| s.to_string()).collect();
}

fn is_valid_number(num: &str) -> bool {
    matches!(num.parse::<usize>(), Ok(_n))
}

fn invalid_input(message: &'static str) {
    panic!("Invalid input! {}", message);
}
