#![allow(dead_code)] // allow all of the dead code in the file

pub fn run() {
    // if_statement();
    // loops();
    match_statement();
}

fn match_statement() {
    //* Notes:
    //      - the match statement should return the same value type for all of the cases
    //      - the cases end with , if there was a single statement with no {}
    //      - the cases don't end with , if there was a multiple statements between {}
    //      - match must be exhaustive patters >> meaning that it must cover all of the possible cases
    //      - _ can match any value (default case)
    let x = 25;
    match x {
        num if num > 10 => {
            println!(">10");
        }
        num if num < 10 => {
            println!("<10");
        }
        _ => {
            println!("____");
        }
    }

    let y = match x {
        num if num > 10 => 50,
        num if num < 10 => 5,
        _ => 0,
    };
    println!("y = {}", y);

    let z = match x {
        10 => 50,     // check single value
        20..=30 => 5, // check range of values
        _ => 0,       // default state
    };
    println!("z = {}", z);
}

fn loops() {
    // loop loop
    'a: loop {
        // run forever or till the loop break
        loop {
            let mut x = 0;
            loop {
                x += 1;
                if x >= 100 {
                    println!("loop break!!!! {}", x);
                    break 'a; // break the labeled loop 'a
                }
            }
        }
    }

    // while loop
    let mut x = 0;
    while x < 100 {
        x += 1;
    }
    println!("while break!!!! {}", x);

    // for loop
    for _ in 0..100 {
        // loop for 100 times doing nothing (no index increment even)
    }

    for (i, elem) in (40..=50).enumerate() {
        println!("arr[{}] = {}", i, elem);
    }
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
