#![allow(non_snake_case, dead_code, unused_imports, unused_macros)]

use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
    }

    println!("{}", y / x);
}
