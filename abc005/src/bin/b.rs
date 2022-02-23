#![allow(non_snake_case, dead_code, unused_imports, unused_macros)]

use proconio::input;

fn main() {
    input! {
        N: usize,
        T: [usize; N],
    }

    println!("{}", T.iter().min().unwrap());
}
