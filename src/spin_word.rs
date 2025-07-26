use std::io::{self, BufRead};

pub fn spin_word_display() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    println!("w:");
    let words = lines.next().unwrap().unwrap().parse::<String>().unwrap();
    println!("fn {:?}", spin_word_fn(words));
}

fn spin_word_fn(words: String) -> String {
    format!("w:{:?}", words.split(' ').collect::<Vec<&str>>())
}

fn words_len(words: String) -> Result<i32, String> {
    match words.trim().is_empty() {
        true => Err(String::from("Description words must be empty ")),
        _ => Ok(words.len() as i32),
    }
}
