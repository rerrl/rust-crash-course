// functions - Used to store blocks of code for reuse

pub fn run(){
    greeting("hello", "Jane");

    //  bind function values to variables
    let get_sum = add(5,5);
    println!("Sum: {}", get_sum);

    // closure: |[input vars]|{[return val]}
    let add_nums = |n1: i32, n2: i32| n1 + n2;
    println!("Closure Sum: {}", add_nums(6,9));
    // you can also do this
    let n3: i32 = 10;
    let add_nums2 = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Closure Sum 2: {}", add_nums2(6,9));

}

fn greeting(greet: &str, name: &str){
    println!("{} {}, nice to meet you!", greet, name);
}

// returns i32
fn add(n1: i32, n2: i32) -> i32 {
    // no semicolon to designate this as the return statement
    n1 + n2
}
