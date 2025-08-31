use std::io::{self, BufRead, Result, stdin};

pub fn two_square_matrix_fn() -> io::Result<()> {
    let stdin = stdin();
    let mut lines = stdin.lock().lines();

    println!("enter the first vector:");

    let vec_1 = match lines.next() {
        Some(Ok(l)) => l
            .trim()
            .split_whitespace()
            .map(|v| {
                v.parse::<i32>().map_err(|e| {
                    io::Error::new(io::ErrorKind::InvalidInput, format!("error : {}", e))
                })
            })
            .collect::<Result<Vec<i32>>>()?,
        Some(Err(e)) => return Err(e)?,
        None => return Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid input")),
    };

    let vec_2 = match lines.next() {
        Some(Ok(l)) => l
            .trim()
            .split_whitespace()
            .map(|v| {
                v.parse::<i32>().map_err(|e| {
                    io::Error::new(io::ErrorKind::InvalidInput, format!("error : {}", e))
                })
            })
            .collect::<Result<Vec<i32>>>()?,
        Some(Err(e)) => return Err(e),
        None => return Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid input")),
    };

    let a = vec![vec![1, 2, 3], vec![3, 2, 1], vec![1, 1, 1]];

    let b = vec![vec![2, 2, 1], vec![3, 2, 3], vec![1, 1, 3]];

    let result = two_square_matrix_result(&a, &b);
    println!("{:?}", result);

    Ok(())
}

fn two_square_matrix_result(vec_1: &Vec<Vec<i32>>, vec_2: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    vec_1
        .iter()
        .zip(vec_2.iter())
        .map(|(row_a, row_b)| row_a.iter().zip(row_b.iter()).map(|(x, y)| x + y).collect())
        .collect()
}
