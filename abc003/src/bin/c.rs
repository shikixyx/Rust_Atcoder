#![allow(non_snake_case, dead_code, unused_imports, unused_macros)]

use proconio::input;

fn main() {
    input!{
        n: usize,
        k: usize,
        mut r: [usize; n],
    }

    r.sort();
    // let mut ans = 0.0;

    // for i in n-k..n{
    //     ans = (ans + r[i] as f64) / 2.0;
    // }
    let ans = r[n-k..]
                .iter()
                .fold(0.0, |acc, &x| { (acc + x as f64) / 2.0});
        

    println!("{}",ans);

}
