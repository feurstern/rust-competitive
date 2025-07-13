fn division_operation(x: i32, y: i32) -> Result<f32, &'static str> {
    match y {
        0 => Err("Cannot be divided by 0!"),
        _ => Ok(x as f32 / y as f32),
    }
}

pub fn error_handling_print() {
    let division_result = division_operation(12, 7).unwrap_or(0.0);
    let division_result_wrap = division_operation(4, 12).unwrap_or_else(|err| {
        println!("Error : {}", err);
        0.0
    });

    println!("divsion result : {}", division_result);
    println!("division result wrap : {}", division_result_wrap);
}
