// vectors are resizable arrays

use std::mem;

pub fn run() {
    // set data type and length of array
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    // reassing value
    numbers[2] = 30;

    println!("{:?}", numbers);

    // get single value
    println!("single value: {}", numbers[0]);

    // get array length
    println!("vector length: {}", numbers.len());

    // arrays are stack allocated
    println!("vector occupies {} bytes", mem::size_of_val(&numbers));

    // get slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

    // add on to vectors
    numbers.push(6);
    numbers.push(7);
    println!("pushed {:?}", numbers);

    numbers.pop();
    println!("popped {:?}", numbers);


    // loop through vector values
    for x in numbers.iter(){
        println!("number {}", x);
    }

    // loop and mutate values (basically js .map)
    for x in numbers.iter_mut(){
        *x *= 2;
    }
    println!("js map equiv {:?}", numbers);
}
