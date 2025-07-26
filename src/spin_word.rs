use std::{
    io::{self, BufRead},
    vec,
};

use serde_json::Error;

pub fn spin_word_display() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    println!("w:");

    let words = lines.next().unwrap().unwrap().parse::<String>().unwrap();
    println!("fn {:?}", spin_word_fn(&words));

    println!("result spin word :{}", spin_word_fn(&words));
}

fn spin_word_fn(words: &String) -> String {
    words
        .split_whitespace()
        .map(|w| {
            if w.len() >= 5 {
                w.chars().rev().collect::<String>()
            } else {
                w.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn words_len(words: String) -> Result<i32, String> {
    match words.len() {
        0 => Err(String::from("Description must be not empty")),
        _ => Ok(words.len() as i32),
    }
}

fn spin_words(words: &str) -> String {
    words
        .split_ascii_whitespace()
        .map(|word| match word.len() >= 5 {
            true => word.chars().rev().collect(),
            false => word.to_string(),
        })
        .collect::<Vec<String>>()
        .join(" ")
}
