use crate::{
    borrower_ex::borrower_ex_display,
    break_camel_case::break_camel_case_fn,
    control_flow::control_flow_print,
    duplicat_encoder::duplicate_encoder_fn,
    enums::enums_print,
    error_handling::error_handling_print,
    ex_variable::ex_variable_print,
    factorial::factorial_display,
    find_intersection::find_intersection_display,
    guess_number::guess_number_print,
    is_this_triangle::is_this_triangle_display,
    looping::looping_print,
    mut_borrow::mut_borrow_print,
    persistent_bugger::persistent_bugger_display,
    phone_number::create_phone_number_display,
    rev_2::rev_2_display,
    rv_a::rev_a_print,
    spin_word::spin_word_display,
    square_digit::square,
    sum_pairs::sum_pairs_fn,
    two_square_matrix::two_square_matrix_fn,
    two_sum::two_sum_diplay,
    variable::{shadowing_variable_print, variable_operation, varialbe_print},
    vector_variable::vector_print,
    verify_vaccine_code::insert_vaccine_id,
    who_like_it::who_likes_display,
};

mod borrower_ex;
mod break_camel_case;
mod control_flow;
mod duplicat_encoder;
mod enums;
mod error_handling;
mod ex_variable;
mod factorial;
mod find_intersection;
mod guess_number;
mod input_rev;
mod is_this_triangle;
mod looping;
mod model;
mod mut_borrow;
mod persistent_bugger;
mod phone_number;
mod remove_odd;
mod rev_2;
mod rv_a;
mod spin_word;
mod square_digit;
mod sum_pairs;
mod two_square_matrix;
mod two_sum;
mod unique_in_order;
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
    // borrower_ex_display();

    // rev_a_print();
    // mut_borrow_print();
    // insert_vaccine_id();t
    // guess_number_print();
    // println!("square result:{}", square(9191));
    // find_intersection_display();
    // factorial_display();
    // who_likes_display();
    // persistent_bugger_display();
    // is_this_triangle_display();
    // rev_2_display();
    // spin_word_display();
    // create_phone_number_display();
    // two_sum_diplay();
    // break_camel_case_fn();

    // duplicate_encoder_fn();
    // sum_pairs_fn();
    two_square_matrix_fn();
}
