#![allow(non_snake_case, dead_code, unused_imports, unused_macros)]

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        W: Chars,
    }

    let mut ans = Vec::new();
    let boin = vec!['a', 'i', 'u', 'e', 'o'];
    for w in W {
        if !boin.contains(&w) {
            ans.push(w);
        }
    }

    println!("{}", ans.into_iter().collect::<String>());
}
