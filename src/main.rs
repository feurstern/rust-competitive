use crate::{
    control_flow::control_flow_print, enums::enums_print, error_handling::error_handling_print, ex_variable::ex_variable_print, factorial::factorial_display, find_intersection::find_intersection_display, guess_number::guess_number_print, is_this_triangle::is_this_triangle_display, looping::looping_print, mut_borrow::mut_borrow_print, persistent_bugger::persistent_bugger_display, rv_a::rev_a_print, square_digit::square, variable::{shadowing_variable_print, variable_operation, varialbe_print}, vector_variable::vector_print, verify_vaccine_code::insert_vaccine_id, who_like_it::who_likes_display
};

mod control_flow;
mod enums;
mod error_handling;
mod ex_variable;
mod factorial;
mod find_intersection;
mod guess_number;
mod is_this_triangle;
mod looping;
mod model;
mod mut_borrow;
mod persistent_bugger;
mod remove_odd;
mod rv_a;
mod square_digit;
mod variable;
mod vector_variable;
mod verify_vaccine_code;
mod who_like_it;

fn main() {
    // println!("Hello, world!");
    // varialbe_print();

    // shadowing_variable_print();
    // variable_operation();

    // ex_variable_print();

    // vector_print();
    // looping_print();
    // control_flow_print();
    // error_handling_print();
    // enums_print();

    // rev_a_print();
    mut_borrow_print();
    // insert_vaccine_id();
    // guess_number_print();
    // println!("square result:{}", square(9191));
    // find_intersection_display();
    // factorial_display();
    // who_likes_display();
    // persistent_bugger_display();
    is_this_triangle_display();
}
