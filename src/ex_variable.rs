pub fn division(x: i32, y: i32) -> Result<f32, &'static str> {
    match y {
        0 => Err("Cannot be divided by zero"),
        _ => Ok(x as f32 / y as f32),
    }
}

pub fn twice_the_number(x: i32) -> i32 {
    x * 2
}

pub fn shadowing_variable_ex() {
    let number = {
        let x = 5;
        let y = 2;
        let x = twice_the_number(x);
        let x = x + y;
        x
    };
    let rust_icon = "🦀";

    println!("rust_icon : {}", rust_icon);
    println!("variable x {}", number)
}

pub fn ex_variable_print() {
    let result_division = match division(4, 6) {
        Ok(r) => r,
        Err(e) => {
            println!("Error : {}", e);
            return;
        }
    };

    shadowing_variable_ex();

    let division_result = match variable_division(4, 10) {
        Ok(r) => r,
        Err(e) => {
            println!("error :{}", e);
            return;
        }
    };

    println!("division result {}", division_result);
    println!("result division:{}", result_division);
}

fn variable_division(x: i32, y: i32) -> Result<f32, &'static str> {
    match y {
        0 => Err("Cannot be divided by zero!"),
        _ => Ok(x as f32 / y as f32),
    }
}
