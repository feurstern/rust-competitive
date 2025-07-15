pub fn mut_borrow_print() {
    let username: String = String::from("user_1");

    println!("username: {}", username); // output user_1

    // now I transfer the ownersnjip to new variable below:
    let username2 = username;
    println!("username 2: {}", username2);

    // it will cause an error when trying to access the username
    // the variable  username is free or deallocated, and you no longer can access it
    // println!("username:{}", username);

    let mut message = String::from("rust is easy man!");

    println!("message: {}", message);

    let new_message = &message;

    println!("new message : {}", new_message);

    // the message is being modified with the given paramter
    change_string(&mut message, String::from("I will become the master!"));

    println!("message after change : {}", message);
}

fn calculate_length_string(s: &String) -> usize {
    s.len()
}

fn change_string(s: &mut String, m: String) {
    s.push_str(&m);
}
