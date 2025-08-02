pub fn two_sum_diplay() {}

fn two_sum_fn(x: i32, y: i32) -> Result<i32, String> {
    match x > y {
        false => Err(String::from("eRROR")),
        true => Ok(x + y),
    }
}
