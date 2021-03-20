//* Conditionals - Used to check the condition of something and act on the result

pub fn run() {
    let age: u8 = 30;
    let check_id: bool = true;
    let knows_person_of_age = false;

    // If/Else
    if ((age >= 21) && check_id) || knows_person_of_age {
        println!("Bartender: What would you like to drink?");
    } else if (age < 21) && check_id {
        println!("Bartender: Sorry, you have to leave");
    } else {
        println!("Bartender: I'll need to see your ID");
    }

    // Shorthand If (like ternary operation)
    let is_of_age: bool = if age >= 21 { true } else { false };
    println!("Is Of Age: {}", is_of_age)

    //* Match (like switch cases) >> see : https://doc.rust-lang.org/rust-by-example/flow_control/match.html
    
    let number = 13;
    
    println!("Tell me about {}", number);
    match number {
        // Match a single value
        1 => println!("One!"),
        
        // Match several values
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        
        // Match an inclusive range
        13..=19 => println!("A teen"),
        //* Handle the rest of cases (Default Case) 
        _ => println!("Ain't special")
    }

    let boolean = true;
    //* Match is an expression too
    let binary = match boolean {
        //* The arms of a match must cover all the possible values
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary);
}
