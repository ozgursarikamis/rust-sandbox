pub(crate) fn if_else_condition(number: i32) {
    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }
}

pub(crate) fn return_conditionally() -> i32 {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    number
}