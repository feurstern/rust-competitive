use ::std::io::{self, BufRead};
pub fn rev_2_display() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    println!("enter the value of n");
    let num = lines.next().unwrap().unwrap().parse::<i32>().unwrap();

    println!("factorial result :{}", rev_factorial(num));
}

fn rev_factorial(num: i32) -> i32 {
    match num {
        0 => 1,
        _ => num * rev_factorial(num - 1),
    }
}
