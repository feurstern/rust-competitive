use crate::{
    control_flow::control_flow_print, error_handling::error_handling_print, ex_variable::ex_variable_print, looping::looping_print, variable::{shadowing_variable_print, variable_operation, varialbe_print}, vector_variable::vector_print
};

mod control_flow;
mod error_handling;
mod ex_variable;
mod looping;
mod model;
mod variable;
mod vector_variable;
fn main() {
    // println!("Hello, world!");
    varialbe_print();

    shadowing_variable_print();
    variable_operation();

    ex_variable_print();

    vector_print();

    looping_print();

    control_flow_print();

    error_handling_print();
}
