use std::io::{self, BufRead};
use std::vec;
struct EducationBackground {
    education_name: &'static str,
    education_level: &'static str,
    graduated_at: &'static str,
    score: f32,
}

struct Address {
    country: &'static str,
    province: &'static str,
    city: &'static str,
    district: &'static str,
    sub_district: &'static str,
    address_detal: &'static str,
}

struct Employe {
    name: &'static str,
    education_background: EducationBackground,
    age: i32,
    address: Address,
}

pub fn division(x: i32, y: i32) -> Result<f32, String> {
    match y {
        0 => Err(String::from("Cannot be divided by0")),
        _ => Ok(x as f32 / y as f32),
    }
}

fn insert_vector() {
    let mut v = String::new();
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
}

pub fn vector_print() {
    let division_result = match division(4, 8) {
        Ok(r) => r,
        Err(e) => {
            println!("Err : {}", e);
            return;
        }
    };
    println!("division result {} :", division_result);

    let data = vec![
        Employe {
            name: "Hatsune Miku",
            age: 17,
            address: Address {
                country: "Japan",
                province: "Ohime",
                city: "Tokyo",
                district: "Tkyo",
                sub_district: "Tokyo",
                address_detal: "Tokyo minato",
            },
            education_background: EducationBackground {
                education_name: "Universitiy Tshin Hua",
                education_level: "Master degree",
                graduated_at: "23 November 1998",
                score: 3.98,
            },
        },
        Employe {
            name: "Haris Wijaya",
            age: 17,
            address: Address {
                country: "Japan",
                province: "Ohime",
                city: "Tokyo",
                district: "Tkyo",
                sub_district: "Tokyo",
                address_detal: "Tokyo minato",
            },
            education_background: EducationBackground {
                education_name: "Universitiy Tshin Hua",
                education_level: "Master degree",
                graduated_at: "23 November 1998",
                score: 3.98,
            },
        },
    ];

    for x in data {
        println!("Name: {}", x.name);
        println!("Age: {}", x.age);
        println!("education :{}", x.education_background.education_name);
    }

    tuple_print();
}

pub fn tuple_print() {
    let random_tuple = ("glory for the nazis", 10, vec![10, 12, 32], true);

    println!("first item of the tuple {:?}", random_tuple.0);
}
