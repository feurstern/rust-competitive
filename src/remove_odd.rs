use std::{
    io::{self, BufRead},
};

pub fn remove_odd_print() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    println!("enter the value:");
    let n = lines.next().unwrap().unwrap().parse::<u32>().unwrap();

    let result = match remove_odd_func(n) {
        Ok(r) => r,
        Err(s) => {
            println!("error : {}", s);
            return;
        }
    };

    println!("result :{}", result)
}


fn remove_odd_func(n: u32) -> Result<u32, String> {
    match n {
        0 => Err(String::from("must be greater 0 ")),
        _ => Ok(n
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap().pow(2).to_string())
            .collect::<String>()
            .parse()
            .unwrap()),
    }
}
