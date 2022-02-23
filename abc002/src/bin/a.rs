#![allow(non_snake_case, dead_code, unused_imports, unused_macros)]

use proconio::input;

fn main() {
    input! {
        X: u32,
        Y: u32,
    }

    println!("{}", std::cmp::max(X, Y));
}
