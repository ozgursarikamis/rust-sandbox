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

pub(crate) fn looping() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        println!("\t {}", counter);

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

pub(crate) fn labeling_loops() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);
}

pub(crate) fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!!!");
}

pub(crate) fn iterating_collection() {
    let a = [1, 2, 3];
    let mut index = 0;

    while index < a.len() {
        println!("a[{}] = {}", index, a[index]);
        index += 1;
    }
}

pub(crate) fn elements_in_array() {
    let a = [1, 2, 3, 4, 5];
    // for element in a.iter() {
    //     println!("The value is {}", element);
    // }
    for element in a {
        println!("The value is {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
}