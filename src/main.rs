use crate::{
    ex_variable::ex_variable_print,
    variable::{shadowing_variable_print, variable_operation, varialbe_print},
};
mod ex_variable;
mod model;
mod variable;
fn main() {
    println!("Hello, world!");
    varialbe_print();

    shadowing_variable_print();
    variable_operation();

    ex_variable_print();
}
