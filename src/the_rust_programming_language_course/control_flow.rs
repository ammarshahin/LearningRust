pub fn run() {
    if_statement();
}

fn if_statement() {
    let mut temp = String::new(); // make a dynamically allocated variable guess(a mutable variable)
    std::io::stdin() // call the standard library
        .read_line(&mut temp) // Read the input string from the stdin and put it into guess variable
        .expect("Failed to read line"); // Handling Potential Failure with the Result Type

    //* Rust allows us to shadow the previous value of guess with a new one. This feature is often used in situations in which you want to convert a value from one type to another type. Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables
    //* The trim method on a String instance will eliminate any whitespace at the beginning and end (eliminates the \n)
    //* The parse method on strings parses a string into some kind of number >> the parse method returns a Result enum type {Ok, Err}
    //* Switching from an expect call to a match expression is how you generally move from crashing on an error to handling the error
    //* The underscore, _, is a catchall value
    let temp: u32 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input!!!");
            0
        }
    };

    if temp > 30 {
        println!("It's really hot here!!!");
    } else if temp >= 20 && temp <= 30 {
        println!("the weather is very nice");
    } else {
        println!("It's really cold here!!!");
    }

    let day = if temp > 20 { "sunny" } else { "cloudy" };
    println!("Today is {}", day);
}
