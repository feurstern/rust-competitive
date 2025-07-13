struct Product {
    id: i32,
    name: String,
    price: f32,
    quantity: i32,
}

enum Membership {
    NonMember,
    Regular,
    Bronze,
    Silver,
    Gold,
    Diamond,
}

fn determine_customer_level(point: i32) -> Membership {
    match point {
        100..=250 => Membership::Regular,
        251..=450 => Membership::Bronze,
        451..=650 => Membership::Silver,
        651..=850 => Membership::Gold,
        851..=1000 => Membership::Diamond,
        _ => Membership::NonMember,
    }
}

fn check_customer_level(member_level: Membership) {
    match member_level {
        Membership::Bronze => println!("Bronze"),
        Membership::Diamond => println!("Diamond"),
        Membership::Regular => println!("Regular"),
        Membership::Silver => println!("Silver"),
        Membership::NonMember => print!("Non member"),
        Membership::Gold => println!("Gold"),
        _ => println!("No membership found"),
    }
}

fn product_discount_calculation(price: f32, discount: f32) -> Result<f32, String> {
    match discount {
        0.0 => Err(String::from("cannot be divided by zero!")),
        _ => Ok(price - (price * discount)),
    }
}

fn product_print() {
    let products = vec![
        Product {
            id: 1,
            name: "Hatsune miku Nendroid".to_string(),
            price: 3500.00,
            quantity: 2,
        },
        Product {
            id: 2,
            name: "Hu Tao Figure".to_string(),
            price: 2500.00,
            quantity: 100,
        },
    ];

    for x in products {
        if x.id % 2 == 0 {
            println!("Odd");
        } else {
            println!("even");
        }
        println!(" Product name: {}", x.name);
        println!("Price: {}", rupiah_format(x.price));
        println!("quantiy:{}", x.quantity);
        let discounted_price = match product_discount_calculation(x.price, 0.05) {
            Ok(f) => f,
            Err(e) => {
                println!("error :{}", e);
                return;
            }
        };

        println!("discounted price: {}", rupiah_format(discounted_price));
    }
}

fn rupiah_format(price: f32) -> String {
    let mut format_price: String = "Rp. ".to_owned();
    format_price.push_str(&price.to_string());

    format_price
}

pub fn rev_a_print() {
    product_print();

    let user_point = determine_customer_level(900);

    check_customer_level(user_point);
}
