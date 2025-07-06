use std::vec;

#[derive(Debug)]
struct User {
    id: i32,
    name: String,
    age: u32,
    job: Jobs,
}

pub fn varialbe_print() {
    // using mut to make the variable mutable
    let mut x = 5;
    let y = "this is stirng";

    println!("the length of string : {}", y.len());
    const PLATFORM_FEE: i32 = 100;

    println!("This is x value : {} and y : {}", x, y);

    // trying to change the value of x
    x = 4;

    print!("new value of x {}", x);

    println!("the platform fee: {}  ", PLATFORM_FEE)
}

pub fn shadowing_variable_print() {
    let z = 100;

    let z = z * 2;

    // shadowwing part
    {
        let z = z + 1;
        println!("The value of z inner scope : {}", z)
    }

    println!("The value of z outside the scope: {}", z)
}

// pub fn variable_scalar_type() {
//     let float_number: f32 = 3.4;

//     let float_number_2: f32 = float_number * 2.0;

//     println!("float number : {} ", float_number_2);

//     // boolean:
//     let mut is_the_main_account: bool = true;

//     is_the_main_account = false;

//     println!("is the main account : {}", is_the_main_account)
// }

fn variable_addiiton(x: i32, y: i32) -> i32 {
    x + y
}

fn variable_division(x: i32, y: i32) -> Result<f32, String> {
    match y {
        0 => Err(String::from("Cannot divide by zero")),
        _ => Ok(x as f32 / y as f32),
    }
}

pub fn discount_calculation(price: f32, discount: f32) -> Result<f32, String> {
    match discount {
        d if d <= 0.0 => Err(String::from("The discount value must be higher than 0")),
        _ => Ok(price - (price * discount)),
    }
}

pub fn time_two(v: i32) -> Result<i32, String> {
    match v {
        v if v <= 0 => Err(String::from("Error must be higher than 0")),
        _ => Ok(v * 2),
    }
}

#[derive(Debug)]
struct Jobs {
    name: &'static str,
    department: &'static str,
    level: &'static str,
}

pub fn variable_operation() {
    let sum = variable_addiiton(4, 6);

    match variable_division(4, 0) {
        Ok(r) => println!("Result: {}", r),
        Err(e) => println!("error {}", e),
    };

    let tup: (i32, i32, &str) = (23, 11, "Hatusne miku");

    println!("tup :{:}", tup.2);

    println!("THE RESULT {}", sum);

    let final_price = match discount_calculation(5000000.0, 0.05) {
        Ok(r) => r,
        Err(e) => {
            println!("Error : {}", e);
            return;
        }
    };

    println!("final+price {}", final_price);
    compound_type();

    let user = vec![
        User {
            id: 1,
            name: String::from("Hatsune Mik"),
            age: 17,
            job: Jobs {
                name: ("Programmer"),
                department: ("IT"),
                level: ("Entry level"),
            },
        },
        User {
            id: 2,
            name: String::from("Hitler"),
            age: 15,
            job: Jobs {
                name: ("Manager"),
                department: ("IT Department"),
                level: ("Supervisor"),
            },
        },
    ];

    for x in &user {
        println!(
            "ID : {}, Name : {}, Age: {}, Job: {}, Department : {}, Level : {}",
            x.id, x.name, x.age, x.job.name, x.job.department, x.job.level
        )
    }

    shadowing_variable();
}

pub fn compound_type() {
    let arr = [1, 2, 3, 4, 5, 3, 1];

    for x in arr {
        println!("value : {}", x as f32 / 2.0);
    }

    // let languages = ["Typescript", "Rust", "Go", "C++"];

    let number = {
        let second_number = 9;
        second_number + 2
    };

    println!("number : {}", number)
}

pub fn shadowing_variable() {
    let number = {
        let x = 9;
        let y = 10;
        let x = match time_two(x) {
            Ok(x) => x,
            Err(e) => {
                println!("error {}", e);
                return;
            }
        };
        let x = x + y;
        x
    };

    println!("shaodwiing variable x: {}", number);
}
// https://www.codewars.com/kata/525f4206b73515bffb000b21/train/javascript
