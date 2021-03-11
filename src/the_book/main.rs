fn main() {
    // print a line
    println!("Guess the number!");
    println!("Please input your guess.");

    // Declare a variable
    let foo = 5; // immutable variable
    let mut bar = 5; // mutable variable
    let mut guess = String::new(); // make a dynamically allocated variable guess(a mutable variable)

    std::io::stdin() // Read the input string from the stdin and put it into guess variable
        .read_line(&mut guess) //  
        .expect("Failed to read line"); // Handling Potential Failure with the Result Type

    // {:?} is the debug trait (the placeholder to print a full array)
    // {0}, {1} ... is used to identify which parameters are used to print
    // {} is a placeholder that is the default.
    println!("You guessed: {}", guess);
}
