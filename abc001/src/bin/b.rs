#![allow(non_snake_case, dead_code, unused_imports, unused_macros)]

use proconio::input;

fn getVV(dist: usize) -> String{
    if dist < 100 {
        "00".to_string()
    } else if dist <= 5000{
        format!("{:02}",(dist / 100))
    } else if dist <= 30000{
        ((dist / 1000) + 50).to_string()
    } else if dist <= 70000{
        ((((dist / 1000) - 30) / 5) + 80).to_string()
    } else {
        "89".to_string()
    }
}

fn main() {
    input!{
        m: usize,
    }
    
    println!("{}",getVV(m))
    
}
