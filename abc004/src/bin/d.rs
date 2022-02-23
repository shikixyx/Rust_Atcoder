#![allow(non_snake_case, dead_code, unused_imports, unused_macros)]

use proconio::input;

fn calc_step(r: isize, mid: isize, total: isize) -> isize {
    let mut t = 0;
    let l = r - total + 1;

    //真ん中にある
    if l <= mid && mid <= r {
        let right = r - mid;
        let left = mid - l;

        t += right * (right + 1) / 2;
        t += left * (left + 1) / 2;
    } else {
        t += (r + l - mid * 2).abs() * (r - l + 1) / 2;
    }

    return t;
}

fn main() {
    input! {
        R: isize,
        G: isize,
        B: isize,
    }

    //println!("{}", calc_step(2499, 2100, 300));

    // 座標を R:1900 G:2000 B:2100 として考える

    let mut ans = 100_000_000;

    for g_start in 1000..3000 {
        let g_end = g_start + G - 1;
        let r_end = g_start - 1;
        let b_start = g_end + 1;
        let b_end = b_start + B - 1;

        let mut t = 0;
        t += calc_step(g_end, 2000, G);

        // println!(
        //     "r_end:{}, g_start:{}, g_end:{}, b_end:{}, t:{}",
        //     r_end, g_start, g_end, b_end, t
        // );

        if (1900 + ((R - 1) / 2)) < r_end {
            t += calc_step(1900 + (R - 1) / 2, 1900, R);
        } else {
            t += calc_step(r_end, 1900, R);
        }

        // println!("R: {}", t);

        if b_start < (2100 - ((B - 1) / 2)) {
            t += calc_step(2100 + (B - 1) / 2, 2100, B);
        } else {
            t += calc_step(b_end, 2100, B);
        }

        // println!("B: {}", t);

        // println!("t = {}", t);
        // println!("");

        ans = ans.min(t);
    }

    println!("{}", ans);
}
