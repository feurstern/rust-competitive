pub fn is_this_triangle_display() {
    let result = is_triangle(4, 2, 1);
    println!("is this triangle:{}", result);
}

fn is_triangle(a: u32, b: u32, c: u32) -> bool {
    if a <= 0 || b <= 0 || c <= 0 {
        false
    } else {
        a + b > c && a + c > b && b + c > a
    }
}
