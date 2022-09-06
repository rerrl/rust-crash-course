/*
Primitive types:
integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
floats: f32, f64
boolean (bool)
characters (char)
tuples
arrays

Rust is a statically typed language! (but the compiler can infer the type)
*/

pub fn run() {
    // default is "i32"
    let x = 1;

    // default is "f64"
    let y = 2.5;

    // add explicit type
    let z: i64 = 565541854148484;

    // get max values
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    let is_active = true;
    let is_greater = 10 < 5;

    // char is single quote like in cpp
    let a1 = 'a';

    // use unicode char
    let face = '\u{1F600}';
    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}
