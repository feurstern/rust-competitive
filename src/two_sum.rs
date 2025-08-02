use ::std::io::{self, BufRead};
use std::collections::HashMap;

pub fn two_sum_diplay() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    println!("Enter target (ex: 2, 100, 50)");

    let target: i32 = match lines.next() {
        Some(Ok(line)) => line.trim().parse().map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("invalid integer for target: {}", e),
            )
        })?,
        Some(Err(e)) => return Err(e),
        None => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "No input provdided",
            ));
        }
    };

    let x: i32 = match lines.next() {
        Some(Ok(line)) => line.trim().parse().map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Invalid target for x : {}", e),
            )
        })?,
        Some(Err(e)) => return Err(e),
        None => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("no input provided"),
            ));
        }
    };

    println!("Enter number sperated by spaces (eg 2  10 11 271)");

    let numbers: Vec<i32> = match lines.next() {
        Some(Ok(line)) => line
            .trim()
            .split_whitespace()
            .map(|s| {
                s.parse::<i32>().map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("invalid input for the arraty: {}", e),
                    )
                })
            })
            .collect::<Result<Vec<i32>, _>>()?,
        Some(Err(e)) => return Err(e),
        None => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("No input provided"),
            ));
        }
    };

    println!("target : {}, x: {}", target, x);

    match two_sum_fn(&numbers, target) {
        Ok((i, j)) => println!("result : [{}, {}]", i, j),
        Err(e) => println!("Error : {}", e),
    }
    Ok(())
}

fn two_sum_fn(numbers: &[i32], target: i32) -> Result<(usize, usize), &'static str> {
    let mut num_map: HashMap<i32, usize> = HashMap::new();

    for (i, &num) in numbers.iter().enumerate() {
        let complement = target - num;

        if let Some(&j) = num_map.get(&complement) {
            return Ok((j, i));
        }

        num_map.insert(num, i);
    }

    unreachable!("failed");
}
