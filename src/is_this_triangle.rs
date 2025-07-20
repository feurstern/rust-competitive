pub fn is_this_triangle_display() {
    let result = is_triangle(7, 10, 4);
    println!("is this triangle:{}", result);
}

fn is_triangle(a: i32, b: i32, c: i32) -> bool {
    match (a > 0, b > 0, c > 0) {
        (true, true, true) => a + b > c && a + c > b && b + c > a,
        _ => false,
    }
}
