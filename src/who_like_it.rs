fn generate_likes(names: &[&str]) -> String {
    match names.len() {
        0 => String::from("no one like this"),
        1 => format!("{} like this", names[0]),
        2 => format!("{} and {} like this", names[0], names[1]),
        3 => format!("{}, {} and {} like this", names[0], names[1], names[2]),
        n => format!("{}, {} and {} others like this", names[0], names[1], n - 2),
    }
}

pub fn who_likes_display() {
    let names = ["miku", "roy", "hitler", "tzuyu", "xoxox"];
    let result = generate_likes(&names);

    println!("likes:{}", result);
}

fn likes(names: &[&str]) -> String {
    match names {
        [] => format!("no one likes this"),
        [a] => format!("{} likes this", a),
        [a, b] => format!("{} and {} like this", a, b),
        [a, b, c] => format!("{}, {} and {} like this", a, b, c),
        [a, b, rest @ ..] => format!("{}, {} and {} others like this", a, b, rest.len()),
    }
}
