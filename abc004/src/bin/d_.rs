#![allow(non_snake_case, dead_code, unused_imports, unused_macros)]

use proconio::input;

fn calc_step(l: usize, total: isize) -> usize {
    let total = total as usize;
    let mut t = 0;
    if (l + 1) > total {
        t += l * (l - total + 1 + l) / 2;
    } else {
        let r = total - l - 1;
        t += l * (l + 1) / 2;
        t += r * (r + 1) / 2;
    }

    return t;
}

fn main() {
    input! {
        R: isize,
        G: isize,
        B: isize,
    }

    let mut ans = 100_000_000;

    for g_start in 0..10 {
        for i in 0..2 {
            let mut t = 0;
            t += calc_step(g_start, G);

            let left: isize = if i == 0 {
                -(g_start as isize)
            } else {
                (g_start as isize) - G + 1
            };
            let right: isize = if i == 0 {
                -(g_start as isize) + G - 1
            } else {
                g_start as isize
            };

            println!("left:{} right:{} t:{}", left, right, t);

            // Red
            if -left + (R - 1) / 2 <= 100 {
                t += calc_step(R as usize / 2, R);
            } else {
                t += calc_step((left + 100).abs() as usize + R as usize, R);
            }

            println!("add Red: {}", t);

            if right + (B - 1) / 2 <= 100 {
                t += calc_step(B as usize / 2, B);
            } else {
                t += calc_step((right - 100).abs() as usize + B as usize, B);
            }

            println!("add Blue: {}", t);
            ans = ans.min(t);

            println!("ans: {}", ans);
            println!("");
        }
    }

    // let mut ans = 0;
    // let r = (R - 1) / 2;
    // ans += r * (r + 1) / 2;
    // ans += (R - 1 - r) * (R - r) / 2;

    // let g = (G - 1) / 2;
    // ans += g * (g + 1) / 2;
    // ans += (G - 1 - g) * (G - g) / 2;

    // let b = (B - 1) / 2;
    // ans += b * (b + 1) / 2;
    // ans += (B - 1 - b) * (B - b) / 2;

    println!("{}", ans);
}
