#![allow(non_snake_case, dead_code, unused_imports, unused_macros)]

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input!{
        mut M: [[String;4]; 4],
    }

    for x in M.into_iter().rev(){
        println!("{}",x.into_iter().rev().collect::<Vec<_>>().join(" "));
    }

}
