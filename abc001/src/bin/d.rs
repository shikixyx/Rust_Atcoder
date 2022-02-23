#![allow(non_snake_case, dead_code, unused_imports, unused_macros)]

fn main() {
    proconio::input! {
        n: usize,
        events: [String;n],
    }

    //println!("{:?}",T);
    let mut events = events
        .iter()
        .map(|s| {
            let s: Vec<usize> = s.split('-').map(|e| e.parse::<usize>().unwrap()).collect();
            let x = s[0] / 5 * 5;
            let mut y = (s[1] + 4) / 5 * 5;
            if y % 100 == 60 {
                y = (y / 100 + 1) * 100;
            }

            (x, y)
        })
        .collect::<Vec<(usize, usize)>>();

    events.sort();
    //println!("{:?}", events);

    // let mut starts = vec![0 as usize; 0];
    // let mut ends = vec![0 as usize; 0];

    let mut s = 2500 as usize;
    let mut e = 0 as usize;
    for (start, end) in events {
        if s == 2500 {
            s = start;
            e = end;
        } else if e < start {
            println!("{:04}-{:04}", s, e);
            s = start;
            e = end;
        } else {
            e = std::cmp::max(e, end);
        }
    }

    println!("{:04}-{:04}", s, e);
}
