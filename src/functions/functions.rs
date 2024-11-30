pub fn callable_function(x: i32) {
    println!("callable function");
}

pub fn labeled_measurement(value: i32, label: i32) {
    println!("The measurement is {value}{label}")
}

pub fn statements_expressions() {
    let y = 6;
    let yy = {
        let x = 3;
        x + 1 // yy now returns 4
    };

    println!("The value of y is: {}", yy);
}