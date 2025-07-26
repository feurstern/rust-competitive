use std::io::{self, BufRead, stdin};

pub fn create_phone_number_display() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let x = lines.next().unwrap().unwrap().parse::<String>().unwrap();
    let arr = x
        .chars()
        .map(|f| f.to_digit(10).unwrap())
        .collect::<Vec<u8>>();
    println!(
        "result phone number: {}",
        create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0])
    );
}

fn create_phone_number(numbers: &[u8]) -> String {
    let digits = numbers
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join("");

    format!("({}) {}-{}", &digits[0..3], &digits[3..6], &digits[6..10])
}
