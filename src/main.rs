// use crate::models::{ Person, Company };

use crate::functions::functions::statements_expressions;

mod basics;
mod models;
mod functions;

fn main() {
    functions::functions::callable_function(5);
    statements_expressions();
}
