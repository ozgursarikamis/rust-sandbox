pub mod basics {
    pub(crate) fn another_function_usage() {
        let x = 5 + 5;
        println!("Hello, world!. Result: {}", x);
    }
}

pub(crate) fn data_types_crate() {
    let x = 5 + 5;
    println!("Hello, world!. Result: {}", x);

    println!("Base 10: {}", 69420);
    println!("Base 2 (binary) {:b}", 69420);
    println!("Base 8 (octal) {:o}", 69420);
    println!("Base 16 (hexadecimal) {:x}", 69420);

    // You can right-justify text with a specified width. This will
    // output "    1". (Four white spaces and a "1", for a total width of 5.)
    println!("{number:>1}", number = 1);

    let _logical: bool = true;
    let _a_float: f32 = 1.0;
    let _an_integer = 5f32;

    let _default_float = 3.0; // default
    let _default_integer = 7;

    let mut inferred_type = 11;
    inferred_type = 12;
    println!("inferred_type: {}", inferred_type);

    let my_array: [i64; 6] = [10, 20, 30, 40, 50, 0];
    for element in my_array.iter() {
        println!("element: {}", element);
    }

    let MyTuple = (5u32, 1u8, "hello", true, 'c', -5.04f32);
    println!("{:?}", MyTuple);
}