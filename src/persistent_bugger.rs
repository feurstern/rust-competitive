use std::io::{self, BufRead};

fn persistent(mut num: u64) -> u64 {
    let mut count = 0;

    while num >= 10 {
        num = num
            .to_string()
            .chars()
            .map(|f| f.to_digit(10).unwrap() as u64)
            .product();

        count += 1;
    }

    count
}

pub fn persistent_bugger_display() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    println!("enter the num:");
    let num = lines.next().unwrap().unwrap().parse::<u64>().unwrap();
    println!("test  {}: {}", num, persistent(num))
}
