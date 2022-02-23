#![allow(non_snake_case, dead_code, unused_imports, unused_macros)]

use proconio::input;

fn main() {
    input! {
        xa: isize,
        ya: isize,
        xb: isize,
        yb: isize,
        xc: isize,
        yc: isize,
    }

    let (a1, a2) = (xa - xc, ya - yc);
    let (b1, b2) = (xb - xc, yb - yc);

    println!("{}", (a1 * b2 - a2 * b1).abs() as f64 / 2.0);
}
