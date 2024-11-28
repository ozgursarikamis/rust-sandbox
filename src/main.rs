use crate::models::{ Person };

mod basics;
mod models;

fn main() {
    // In Rust, the pub(crate) visibility modifier allows the item to be accessible within the same crate.
    // This is necessary because main.rs and data_types are part of the same crate,
    // and you want to expose the data_types_crate function to other modules within that crate.
    basics::data_types::data_types_crate();
    basics::data_types::basics::another_function_usage();

    let first_name = String::from("Ozgur");
    let age = 27;
    let ozgur = Person { first_name, age };

    println!("{}|{}", ozgur.first_name, ozgur.age)
}
