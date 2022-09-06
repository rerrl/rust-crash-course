// reference pointer - point to a resource in memory
pub fn run(){
    // primitive array
    let arr1 = [1,2,3];
    let arr2 = arr1;
    println!("values(primitives): {:?}", (arr1, arr2));

    // with non-primiteves,if you assign another variable to a piece of data, the first
    // variable will no longer hold that value. You'll need to use a reference (&) to poin to the
    // resource

    // vector
    let vec1 = vec![1,2,3];
    let vec2 = &vec1; // cant point directly to vec1 if its not a primitive value, you need to create a reference
    println!("values(non primitive): {:?}", (&vec1, vec2));
}
