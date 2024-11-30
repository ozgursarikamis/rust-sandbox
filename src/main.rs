use crate::control_flows::{elements_in_array, if_else_condition, iterating_collection, labeling_loops, looping, return_conditionally, while_loop};

mod basics;
mod control_flows;
// mod functions;

fn main() {
    looping();
    labeling_loops();
    println!("================");
    while_loop();
    println!("================");
    iterating_collection();
    println!("================");
    elements_in_array();

    // https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html
}
