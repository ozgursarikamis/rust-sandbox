use crate::control_flows::{
    if_else_condition,
    return_conditionally
};

mod basics;
mod control_flows;
mod functions;

fn main() {
    if_else_condition(4);
    if_else_condition(6);

    println!("returned number is: {:?}", return_conditionally());
}
