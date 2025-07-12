fn match_colour(rbg: (i32, i32, i32)) {
    let vec_colour = vec![(rbg.0, "red"), (rbg.1, "green"), (rbg.2, "blue")];

    for x in vec_colour {
        println!("colour {} : {} ", x.1, x.0);
    }

    let result_tupple_division = match division_tupple((rbg.0, rbg.1), 10) {
        Ok(r) => r,
        Err(e) => {
            println!("error : {}", e);
            return;
        }
    };

    println!("result tuple divison : {}", result_tupple_division);
}

pub fn looping_print() {
    match_colour((110, 4, 59));
}

pub fn division_tupple(tupple_value: (i32, i32), division: i32) -> Result<f32, &'static str> {
    match division {
        0 => Err("Cannot be divided by 0!"),
        _ => Ok((tupple_value.0 as f32 + tupple_value.1 as f32) / division as f32),
    }
}
