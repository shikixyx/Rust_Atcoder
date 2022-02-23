#![allow(non_snake_case, dead_code, unused_imports, unused_macros)]

use proconio::input;

fn main() {
    input!{
        mut deg: usize,
        dis: usize,
    }

    deg *= 10;

    let dir = if deg < 1125{
        "N"
    } else if deg < 3375{
        "NNE"
    } else if deg < 5625{
        "NE"
    } else if deg < 7875{
        "ENE"
    } else if deg < 10125{
        "E"
    } else if deg < 12375{
        "ESE"
    } else if deg < 14625{
        "SE"
    } else if deg < 16875{
        "SSE"
    } else if deg < 19125{
        "S"
    } else if deg < 21375{
        "SSW"
    } else if deg < 23625{
        "SW"
    } else if deg < 25875{
        "WSW"
    } else if deg < 28125{
        "W"
    } else if deg < 30375{
        "WNW"
    } else if deg < 32625{
        "NW"
    } else if deg < 34875{
        "NNW"
    } else {
        "N"
    };

    let v = (dis as f32/ 6.0).round() / 10.0;

    let w = if v <= 0.2{
        0
    } else if v <= 1.5{
        1
    } else if v <= 3.3{
        2
    } else if v <= 5.4{
        3
    } else if v <= 7.9{
        4
    } else if v <= 10.7{
        5
    } else if v <= 13.8{
        6
    } else if v <= 17.1{
        7
    } else if v <= 20.7{
        8
    } else if v <= 24.4{
        9
    } else if v <= 28.4{
        10
    } else if v <= 32.6{
        11
    } else {
        12
    };

    println!("{} {}",
        match w{
            0 => "C",
            _ => dir
        },
        w
    )
}
