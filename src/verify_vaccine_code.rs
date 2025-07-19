use std::io::{self, BufRead};

pub fn verify_vaccine_id(vaccine_id: &String) -> String {
    if vaccine_id.len() > 12 || vaccine_id.len() < 12 {
        String::from("invalid id!")
    } else {
        String::from("valid id")
    }
}

fn div_test(x: i32, y: i32) -> Result<f32, String> {
    match y {
        0 => Err(String::from("Error cannot divided by 0!")),
        _ => Ok(x as f32 / y as f32),
    }
}

pub fn insert_vaccine_id() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    println!("enter your vaccine id: ");
    let vaccine_id = lines.next().unwrap().unwrap().parse::<String>().unwrap();
    let n = lines.next().unwrap().unwrap().parse::<i32>().unwrap();
    println!("vaccine_id : {}", vaccine_id);

    let verifying_result = verify_vaccine_id(&vaccine_id);

    println!("enter x: value");
    let x = lines.next().unwrap().unwrap().parse::<i32>().unwrap();
    println!("enter y value:");
    let y = lines.next().unwrap().unwrap().parse::<i32>().unwrap();

    let result = match div_test(x, y) {
        Ok(f) => f,
        Err(s) => {
            println!("error:{}", s);
            return;
        }
    };
    println!("result division : {}", result);
    println!("verfying result:{}", verifying_result);
}
