
fn find_intersection_fn(vec_1: Vec<i32>, vec_2: Vec<i32>) -> Vec<i32> {
    vec_1.into_iter().filter(|x| vec_2.contains(x)).collect()
}

pub fn find_intersection_display() {
    let vec_1 = vec![2, 4, 1, 34, 31, 5, 12, 17];
    let vec_2 = vec![2, 1, 17, 9, 11, 15];

    let intesection = find_intersection_fn(vec_1, vec_2);
    println!("result {:?}", intesection);
}
