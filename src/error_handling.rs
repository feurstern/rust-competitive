fn division_operation(x: i32, y: i32) -> Result<f32, &'static str> {
    match y {
        0 => Err("Cannot be divided by 0!"),
        _ => Ok(x as f32 / y as f32),
    }
}

pub fn error_handling_print() {
    let result_division = division_operation(4, 12).unwrap_or(0.0);
    let result_division_wrap = division_operation(10, 8).unwrap_or_else(|err| {
        println!("error:{}", err);
        0.0
    });

    println!("result division : {}", result_division);
    println!("result division with fallbacl : {}", result_division_wrap);
}
