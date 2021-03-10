// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    // Variables are immutable by default == constants
    let name = "ammar";
    println!("my name is {}", name);

    // To modify variables we should use the mut keyword
    let mut age = 26;
    println!("and my age is {}", age);
    age += 1;
    println!("and next year my age will be {}", age);

    // define const
    const ID: i32 = 1;
    println!("my ID is {}", ID);

    // assign multiple vars
    let (my_name, my_age) = ("Ammar", 26);
    println!("my name is {} and my age is {}", my_name, my_age);
}
