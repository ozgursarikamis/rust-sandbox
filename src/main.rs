// use crate::models::{ Person, Company };

use crate::functions::functions::{plus_one, return_values, statements_expressions};

mod basics;
mod models;
mod functions;

fn main() {
    functions::functions::callable_function(5);
    statements_expressions();
    println!("Return values: {}", return_values());

    let mut x = 9;
    x = plus_one(x);
    println!("Return values: {}", x);
}
