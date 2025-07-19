use std::io::{self, BufRead};

fn guess(n: i32) -> bool {
    if n < 1 || n > 10 {
        panic!("invalid number :{}", n)
    }

    n == 5
}

pub fn guess_number_print() {
    let stdin = io::stdin();

    let mut line = stdin.lock().lines();

    let n = line.next().unwrap().unwrap().parse::<i32>().unwrap();

    println!("the result :{}", guess(n));
}
