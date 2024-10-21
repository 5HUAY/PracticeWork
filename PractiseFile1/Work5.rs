use std::io::{self, BufRead};

/*
 * Complete the 'plusMinus' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn plusMinus(arr: &[i32]) {
    let total_count = arr.len() as f64;
    let (positive_count, negative_count, zero_count) = arr.iter().fold((0, 0, 0), |(pos, neg, zer), &num| {
        if num > 0 {
            (pos + 1, neg, zer)
        } else if num < 0 {
            (pos, neg + 1, zer)
        } else {
            (pos, neg, zer + 1)
        }
    });

    let positive_ratio = positive_count as f64 / total_count;
    let negative_ratio = negative_count as f64 / total_count;
    let zero_ratio = zero_count as f64 / total_count;

    println!("{:.6}", positive_ratio);
    println!("{:.6}", negative_ratio);
    println!("{:.6}", zero_ratio);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    plusMinus(&arr);
}
