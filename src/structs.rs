// Structs - used to create custom data types

// traditional struct (capitalized naming is convention)
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

// tuple struct
struct DifferentColor(u8, u8, u8);



// struct with associated functions
struct Person {
    first_name: String,
    last_name: String
}

// "implements the person struct"
impl Person {
    // create new function construct Person
    fn new(first: &str, last: &str) -> Person {
        Person{
            first_name: first.to_string(), // need to_string becuase the the intended type is String
            last_name: last.to_string()
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set last name
    fn set_last_name(&mut self, last: &str){
        self.last_name = last.to_string();
    }

    // name to tuple
    fn to_tuple(self) -> (String, String){
        (self.first_name, self.last_name)
    }
}


pub fn run(){
    // Color struct stuff
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0
    };

    c.red = 200;

    println!("Color: {} {} {}", c.red, c.blue, c.green);


    // DifferentColor stuff
    let mut dc = DifferentColor(255, 0, 0);
    dc.2 = 70;
    println!("DifferentColor: {} {} {}", dc.0, dc.1, dc.2);


    // use the Person object and associated function
    let mut p = Person::new("John", "Cena");
    println!("Person: {} {}", p.first_name, p.last_name);

    println!("Person (full_name): {}", p.full_name());

    p.set_last_name("Steinbeck");
    println!("Person (name change): {}", p.full_name());

    println!("Person (to_tuple): {:?}", p.to_tuple());
}
