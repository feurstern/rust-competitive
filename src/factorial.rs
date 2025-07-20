use std::io::{self, BufRead};

pub fn factorial_display() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    println!("enter n:");
    let n = lines.next().unwrap().unwrap().parse::<i32>().unwrap();
    let result = factorial(n);
    println!("factorial result :{}", result);
}

fn factorial(x: i32) -> i32 {
    match x {
        0 => 1,
        _ => x * factorial(x - 1),
    }
}
