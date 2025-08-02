use ::std::io::{self, BufRead};

pub fn input_rev() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    println!("x");

    let x: i32 = match lines.next() {
        Some(Ok(line)) => line.trim().parse().map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("invalid integer :{}", e),
            )
        })?,
        Some(Err(e)) => return Err(e),
        None => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("No input provided"),
            ));
        }
    };

    println!("array white space");

    let arr: Vec<i32> = match lines.next() {
        Some(Ok(line)) => line
            .trim()
            .split_whitespace()
            .map(|s| {
                s.parse::<i32>().map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("invalid integer:{}", e),
                    )
                })
            })
            .collect::<Result<Vec<i32>, _>>()?,
        Some(Err(e)) => return Err(e),
        None => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("invalid quie"),
            ));
        }
    };

    Ok(())
}
