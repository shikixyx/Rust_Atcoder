#![allow(non_snake_case, dead_code, unused_imports, unused_macros)]

use proconio::input;

fn main() {
    input! {
        T: usize,
        N: usize,
        mut A: [usize; N],
        M: usize,
        mut B: [usize; M],
    }

    let mut flg = true;
    A.sort();
    B.sort();

    while B.len() > 0 {
        let mut ok = false;
        let b = B.pop().unwrap();

        while A.len() > 0 {
            let a = A.pop().unwrap();
            if a <= b && b <= (a + T) {
                ok = true;
                break;
            }
        }

        if !ok {
            flg = false;
            break;
        }
    }

    println!("{}", if flg { "yes" } else { "no" });
}
