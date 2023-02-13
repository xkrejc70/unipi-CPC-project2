mod lib;

use std::io::{self, BufRead};

fn main() {
    let mut arr: Vec<usize> = Vec::new();
    let mut ops: Vec<Vec<usize>> = Vec::new();
    let mut qrs: Vec<Vec<usize>> = Vec::new();
    let mut num_op: Vec<usize> = Vec::new();
    let mut arr_res: Vec<usize> = Vec::new();

    // Parse and validate input
    (arr, ops, qrs) = parse_input();

    let ops_len = ops.len() - 1;
    let arr_len = arr.len() - 1;

    // Create segment trees
    let mut seg_tree_ops: Vec<lib::NodeOp> = lib::create_segment_tree_ops(ops_len);
    let mut seg_tree_arr: Vec<lib::Node> = lib::create_segment_tree_arr(arr);

    // Execute queries
    num_op = lib::execute_queries(&mut seg_tree_ops, qrs, ops_len);

    // Update operation values
    for (i, num) in num_op.iter().enumerate() {
        ops[i][2] *= num;
    }

    // Execute operations
    arr_res = lib::execute_operations(&mut seg_tree_arr, ops, arr_len);

    print_array(arr_res);
}

fn parse_input() -> (Vec<usize>, Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let input_lines: io::Lines<io::StdinLock> = io::stdin().lock().lines();
    let mut line_num: usize = 1;
    let mut op_num: usize = 0;
    let mut qr_num: usize = 0;
    let mut n: usize = 0;
    let mut m: usize = 0;
    let mut k: usize = 0;
    let mut arr: Vec<usize> = Vec::new();
    let mut ops: Vec<Vec<usize>> = Vec::new();
    let mut qrs: Vec<Vec<usize>> = Vec::new();

    for line in input_lines {
        let line = line.unwrap();

        if line.is_empty() {
            break;
        }

        match line_num {
            // 1st line (n, m)
            1 => (n, m, k) = parse_nums(&line),
            // 2nd line (array A)
            2 => arr = parse_array(n, &line),
            // Queries
            _ => {
                if line_num > m + 2 {
                    qrs.push(parse_query(&line, m));
                    qr_num += 1;
                } else {
                    ops.push(parse_operation(&line, n));
                    op_num += 1;
                }
            }
        }

        if op_num > m {
            invalid_input("number of operations");
        }
        if qr_num > k {
            invalid_input("number of queries");
        }

        line_num += 1;
    }

    if op_num < m {
        invalid_input("number of operations");
    }
    if qr_num < k {
        invalid_input("number of queries");
    }

    (arr, ops, qrs)
}

// Parse first line (n, m, k)
fn parse_nums(str: &str) -> (usize, usize, usize) {
    let nums: Vec<String> = split_by_space(str);

    if nums.len() != 3 {
        invalid_input("first line length");
    }

    // Check number format
    if !is_valid_number(&nums[0]) || !is_valid_number(&nums[1]) || !is_valid_number(&nums[2]) {
        invalid_input("number format");
    }

    let n: usize = nums[0].parse().unwrap();
    let m: usize = nums[1].parse().unwrap();
    let k: usize = nums[2].parse().unwrap();

    /*
    commented cuz it would not pass the test
    if k > n || m > n {
        invalid_input("k or m is to large");
    }
    */

    (n, m, k)
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
        array_res.push(elem.parse::<usize>().unwrap())
    }

    array_res
}

// Parse operation line
fn parse_operation(op: &str, n: usize) -> Vec<usize> {
    let op: Vec<String> = split_by_space(op);
    let mut op_res: Vec<usize> = Vec::new();

    if op.len() != 3 {
        invalid_input("operation length");
    }

    for elem in op.iter() {
        if !is_valid_number(elem) {
            invalid_input("not a number");
        }
        op_res.push(elem.parse::<usize>().unwrap());
    }

    let l = op[0].parse::<usize>().unwrap();
    let r = op[1].parse::<usize>().unwrap();

    if l < 1 || r > n {
        invalid_input("operation range")
    }

    op_res
}

// Parse query line
fn parse_query(qr: &str, m: usize) -> Vec<usize> {
    let qr: Vec<String> = split_by_space(qr);
    let mut qr_res: Vec<usize> = Vec::new();

    if qr.len() != 2 {
        invalid_input("query length");
    }

    for elem in qr.iter() {
        if !is_valid_number(elem) {
            invalid_input("not a number");
        }
        qr_res.push(elem.parse::<usize>().unwrap());
    }

    let l = qr[0].parse::<usize>().unwrap();
    let r = qr[1].parse::<usize>().unwrap();

    if l < 1 || r > m {
        invalid_input("query range")
    }

    qr_res
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

fn print_array(arr: Vec<usize>) {
    let str = arr
        .iter()
        .map(|val| format!("{}", val))
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", str);
}
