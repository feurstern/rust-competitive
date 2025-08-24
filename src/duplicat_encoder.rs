use std::{
    collections::HashMap,
    io::{self, BufRead, stdin},
};

pub fn duplicate_encoder_fn() -> io::Result<()> {
    let stdin = stdin();
    let mut lines = stdin.lock().lines();

    let words: String = match lines.next() {
        Some(Ok(l)) => l
            .parse()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, format!("error : {}", e)))?,
        Some(Err(e)) => return Err(e),
        None => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "empty space".to_string(),
            ));
        }
    };

    let mut result = String::new();

    let mut freq = HashMap::new();

    for w in words.to_lowercase().chars() {
        *freq.entry(w).or_insert(0) += 1;
    }

    for w in words.chars() {
        if freq[&w] > 1 {
            result.push(')');
        } else {
            result.push('(');
        }
    }

    println!("{}", result);

    Ok(())
}
