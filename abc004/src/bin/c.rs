#![allow(non_snake_case, dead_code, unused_imports, unused_macros)]

use proconio::input;
use std::collections::VecDeque;

fn main() {
    input!{
        n: usize,
    }

    let rotate = (n / 5) % 6;
    let num = n % 5;

    let mut arr: VecDeque<u64> = (1..=6).collect();
    
    for _ in 0..rotate{
        let a = arr.pop_front().unwrap();
        arr.push_back(a);
    }

    for i in 0..num{
        arr.swap(i, i+1);
    }

    println!("{}",Vec::from(arr).iter().map(|e| e.to_string()).collect::<String>());
}