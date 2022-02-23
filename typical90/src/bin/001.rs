#![allow(non_snake_case, dead_code, unused_imports, unused_macros)]

use proconio::input;

fn main() {
    input! {
        N: usize,
        L: u64,
        K: u64,
        A: [u64; N],
    }

    let mut left: u64 = 0;
    let mut right: u64 = L + 1;

    while (right - left) > 1 {
        let mid = (left + right) / 2;

        let mut cnt = 0;
        let mut pre = 0;

        for i in 0..N {
            let l = A[i] - pre;

            if l >= mid {
                cnt += 1;
                pre = A[i];
            }
        }

        if L - pre >= mid {
            cnt += 1
        }

        if cnt >= K + 1 {
            left = mid
        } else {
            right = mid
        }
    }

    println!("{}", left);
}
