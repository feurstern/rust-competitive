use ::std::io::{self, BufRead};
pub fn rev_2_display() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    println!("enter the value of n");
    let num = lines.next().unwrap().unwrap().parse::<i32>().unwrap();

    println!("enter the value of x");
    let x = lines.next().unwrap().unwrap().parse::<u32>().unwrap();
    println!("factorial result :{}", rev_factorial(num));
    println!("vector {:?}", convert_digit_to_vector(x));
}

fn rev_factorial(num: i32) -> i32 {
    match num {
        0 => 1,
        _ => num * rev_factorial(num - 1),
    }
}

fn convert_digit_to_vector(x: u32) -> Vec<u32> {
    x.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}
