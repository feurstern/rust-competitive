use ::std::io::{self, BufRead};

enum VecResult {
    Postive,
    Negative,
}

pub fn borrower_ex_display() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    println!("enter x :");
    let x = lines.next().unwrap().unwrap().parse::<u32>().unwrap();
    let z = x;
    println!("result digit{:?}", borrow_dg_vec(z));

    let v = vec![1, 3, 4, 5];

    println!("take {:?} :", take(v));
}

fn borrow_dg_vec(x: u32) -> Vec<u32> {
    x.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

fn take(v: Vec<i32>) -> Vec<i32> {
    v.into_iter().map(|c| c.pow(2)).collect()
}
