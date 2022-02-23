#![allow(non_snake_case, dead_code, unused_imports, unused_macros)]

use proconio::input;
use proconio::marker::Chars;

fn main() {
    let rep = vec!['a','t','c','o','d','e','r'];
    input!{
        S: Chars,
        T: Chars,
    }

    let l = S.len();
    let mut flg = true;

    for i in 0..l{
        if S[i] != '@' && T[i] != '@'{
            if S[i] != T[i]{
                flg = false;
            }
        }else if S[i] == '@' && T[i] == '@'{
            continue;
        }else{
            if !(rep.contains(&S[i]) || rep.contains(&T[i])){
                flg = false;
            }
        }
    }

    if flg{
        println!("You can win");
    }else{
        println!("You will lose");
    }

    // input!{
    //     s: String,
    //     t: String,
    // }

    // let is_atcoder = |c : char| "atcoder".chars().any(|d| c == d);
    // let ans = s.chars().zip(t.chars()).all(|(c,d)| {
    //     c == d 
    //     || (c == '@' && is_atcoder(d))
    //     || (is_atcoder(c) && d == '@')
    // });

    // println!("{}",if ans {"You can win"} else {"You will lose"});
}
