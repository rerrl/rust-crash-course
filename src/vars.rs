// variables hold primitive data or references to data
// variables are immutable by default
// rust is a block-scoped language

pub fn run() {
    let name = "tony";
    let mut age = 37;  // add "mut" to make mutable
    println!("my name is {} and i am {}", name, age);
    age = 38;
    println!("my name is {} and i am {}", name, age);

    // define constant (all uppercase variable naming, type assigned)
    const ID: i32 = 001;
    println!("ID: {}", ID);

    let (my_name, my_age) = ("brad", 37);
    println!("{} is {}", my_name, my_age);
}
