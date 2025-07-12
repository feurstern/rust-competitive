fn odd_even_number(x: i32) -> String {
    let result = x % 2;
    match result {
        0 => String::from("The number is even"),
        _ => String::from("The number is oddd"),
    }
}

fn weather_print() {
    let sky = "cloudy";
    let temperature = "warm";

    match (sky, temperature) {
        ("cloudy", "warm") => println!("it will be dark outside"),
        ("clear", "warm") => println!("it will be clear!"),
        _ => println!("Invalid input of sky or temperature!"),
    }
}

fn degree_converter(value: i32, type_degree: &'static str) -> f32 {
    match type_degree {
        "celcius" => value as f32 * 2.0,
        "fahrenheit" => value as f32 * (0.8),
        _ => 0.0,
    }
}

pub fn control_flow_print() {
    let number_type = odd_even_number(4);

    println!("number type : {}", number_type);

    weather_print();

    let degrees = degree_converter(32, "celcius");

    println!("degrees : {}", degrees);
}
