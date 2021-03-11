fn main() {
    // print a line
    println!("Guess the number!");
    println!("Please input your guess.");

    // Declare a variable
    let foo = 5; // immutable variable
    let mut bar = 5; // mutable variable
    let mut guess = String::new(); // make a dynamically allocated variable guess(a mutable variable)

    std::io::stdin() // read the input string from the stdin and put it into guess variable
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess); // {} is a placeholder that is the default.
}
