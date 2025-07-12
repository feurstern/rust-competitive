use crate::{
    ex_variable::ex_variable_print,
    looping::looping_print,
    variable::{shadowing_variable_print, variable_operation, varialbe_print},
    vector_variable::vector_print,
};

mod ex_variable;
mod looping;
mod model;
mod variable;
mod vector_variable;
fn main() {
    println!("Hello, world!");
    varialbe_print();

    shadowing_variable_print();
    variable_operation();

    ex_variable_print();

    vector_print();

    looping_print();
}
