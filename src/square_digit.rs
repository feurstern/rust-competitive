pub fn square(num: u64) -> u64 {
    num.to_string()
        .chars()
        .map(|f| f.to_digit(10).unwrap().pow(2).to_string())
        .collect::<String>()
        .parse::<u64>()
        .unwrap()
}
