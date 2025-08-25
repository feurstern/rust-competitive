use std::{
    io::{self, BufRead, Result, stdin},
    vec,
};

pub fn sum_pairs_fn() -> io::Result<()> {
    let stdin = stdin();
    let mut lines = stdin.lock().lines();

    println!("enter the value seperated by space ");

    let ints = match lines.next() {
        Some(Ok(l)) => l
            .trim()
            .split_whitespace()
            .map(|i| {
                i.parse::<i32>().map_err(|e| {
                    io::Error::new(io::ErrorKind::InvalidInput, format!("error : {}", e))
                })
            })
            .collect::<Result<Vec<i32>>>()?,
        Some(Err(e)) => return Err(e),
        None => {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid input"));
        }
    };

    println!(" ints {:?}", ints);
    Ok(())
}
