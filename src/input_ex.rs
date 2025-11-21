use std::io::{self, Write};

pub fn get_user_input() -> io::Result<()> {
    println!("Enter your name");
    io::stdout().flush()?;
    let mut name = String::new();
    let mut x: i32 = 3;
    io::stdin().read_line(&mut name)?;
    println!("Hello {}!", name);
    Ok(())
}
