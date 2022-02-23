#![allow(non_snake_case, dead_code, unused_imports, unused_macros)]

use proconio::input;

fn main() {
    input!{
        N: usize,
    }

    println!("{}",
        (1..=N).map(|n| n * 10000).sum::<usize>() as f64 / N as f64
    )
}
