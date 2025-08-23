use std::io::{self, BufRead, stdin};

pub fn break_camel_case_fn() -> io::Result<()> {
    let stdin = stdin();
    let mut lines = stdin.lock().lines();

    let string: String = match lines.next() {
        Some(Ok(i)) => i
            .parse()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, format!("error : {}", e)))?,
        Some(Err(e)) => return Err(e),
        None => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "no input given!",
            ));
        }
    };

    let mut result = String::new();

    for c in string.chars() {
        if c.is_ascii_uppercase() {
            result.push(' ');
        }

        result.push(c);
    }

    println!("{}", result);

    Ok(())
}
