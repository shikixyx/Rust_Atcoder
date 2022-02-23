#![allow(non_snake_case, dead_code, unused_imports, unused_macros)]

use proconio::input;
use itertools::Itertools;

fn main() {
    input!{
        N: usize,
        M: usize,
        rel: [(usize,usize); M],
    }

    let mut ans = 1;

    for i in 0..(1 << N){
        let mut group = Vec::new();
        for j in 0..N{
            if (i >> j) & 1 == 1{
                group.push(j);
            }
        }

        if group.len() <= 1{
            continue;
        }

        let mut flg = true;
        for x in group.iter().combinations(2){
            if !rel.contains(&(x[0]+1,x[1]+1)){
                flg = false;
            }
        }

        if flg{
            ans = ans.max(group.len());
        }
    }

    println!("{}",ans);
    

}
