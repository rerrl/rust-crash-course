// primitive string = immutable fixed-length string somewhere in memory
// string = growable, heap-allocated data structure. Use when you need
// to modify your own string data

pub fn run(){
    let mut hello = String::from("Hello ");
    // get length
    println!("Length: {}", hello.len());
    println!("{}", hello);

    // can push a single character to string
    hello.push('W');

    // or you can push a string
    hello.push_str("orld!");
    println!("{}", hello);

    // capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // check if empty
    println!("is empty: {}", hello.is_empty());

    // containts
    println!("containts 'world': {}", hello.contains("World"));

    // replace
    println!("Replace: {}", hello.replace("World","there"));

    // loop through string by whitespace
    for word in hello.split_whitespace(){
        println!("{}", word);
    }

    // create string with capacity:
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{}", s);

    // assertion testing (raises when false)
    assert_eq!(2, s.len());
}
