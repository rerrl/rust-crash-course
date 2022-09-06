use std::env;

// from cli, issue the command: cargo run "hello" 525
// and see what this args variable is storing.
// spoiler, it saves your command line args and puts them in an array.
// first index is the target executable

pub fn run() {
    let args: Vec<String> = env::args().collect();
    // println!("Args: {:?}", args);
    let command = args[1].clone();
    let name = "brad";
    let status = "100%";

    if command == "hello" {
        println!("Hello {}, how are you?", name);
    }else if command == "status" {
        println!("Status is: {}", status);
    } else {
        println!("That is not a valid command");
    }
}
