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

    // words.split(" ").collect::<Vec<&str>>(}
}
