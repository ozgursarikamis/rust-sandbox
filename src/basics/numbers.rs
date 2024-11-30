pub(crate) fn basic_numbers() {
       let x: i32 = 5;
       let mut y: i32 = 5;

       y = x + 1;
       let z: i32 = y; // type of z?
       println!("The value of z is: {}", z);

       let v: u16 = 38_u8 as u16;
       println!("The value of v is: {}", v);
   }

   pub(crate) fn assert_numbers() {
       let x = 5;
       assert_eq!("i32".to_string(), type_of(&x));
       println!("Success")
   }

// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}