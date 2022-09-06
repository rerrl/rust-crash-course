// arrays - fixed list where elements are the same data types

use std::mem;

pub fn run() {
    // set data type and length of array
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // reassing value
    numbers[2] = 30;

    println!("{:?}", numbers);

    // get single value
    println!("single value: {}", numbers[0]);

    // get array length
    println!("array length: {}", numbers.len());

    // arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // get slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);
}
