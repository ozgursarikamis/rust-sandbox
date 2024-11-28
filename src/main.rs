mod basics;

fn main() {
    // In Rust, the pub(crate) visibility modifier allows the item to be accessible within the same crate.
    // This is necessary because main.rs and basics_01_data_types.rs are part of the same crate,
    // and you want to expose the data_types_crate function to other modules within that crate.
    basics::basics_01_data_types::data_types_crate();
    basics::basics_01_data_types::basics::another_function_usage();
}
