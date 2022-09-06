pub fn run() {
    // print to console
    println!("Hello, from the print.rs file!");

    // positional arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "brad", "texas", "code"
    );

    // named arguments
    println!(
        "my name is {name} and I like to {hobby}",
        name = "juan-carlos",
        hobby = "drink coffee"
    );

    // placeholder traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    // placeholder for debug trait (using a tuple)
    println!("{:?}", (12, true, "hello"));

    // basic math
    println!("10 + 10 = {}", 10 + 10);
}
