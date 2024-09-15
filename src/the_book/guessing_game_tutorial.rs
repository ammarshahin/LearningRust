#![warn(clippy::use_debug)]

//*  trait defines methods that random number generators implement
use rand::Rng;

//* Ordering : is an enum {Less, Greater, Equal}
use std::cmp::Ordering;

pub 

pub fn run() {
  // print a line
  println!("Guess the number!");

  //* thread_rng() : will give us the particular random number generator
  //* gen_range() :  it takes two numbers as arguments and generates a random number between them
  // Create and generate the secret number
  let the_secret_number = rand::thread_rng().gen_range(1..101);

  // print the secret number
  dbg!(the_secret_number);

  loop {
    println!("Please input your guess.");
    // Declare a variable
    let mut the_guessed_number = String::new(); // make a dynamically allocated variable guess(a mutable variable)
    std::io::stdin() // call the standard library
      .read_line(&mut the_guessed_number) // Read the input string from the stdin and put it into guess variable
      .expect("Failed to read line"); // Handling Potential Failure with the Result Type

    //* Rust allows us to shadow the previous value of guess with a new one. This feature is often used in situations in which you want to convert a value from one type to another type. Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables
    //* The trim method on a String instance will eliminate any whitespace at the beginning and end (eliminates the \n)
    //* The parse method on strings parses a string into some kind of number >> the parse method returns a Result enum type {Ok, Err}
    //* Switching from an expect call to a match expression is how you generally move from crashing on an error to handling the error
    //* The underscore, _, is a catchall value
    let the_guessed_number: u32 = match the_guessed_number.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        println!("Invalid input!!!");
        continue;
      }
    };

    // {:?} is the debug trait (the placeholder to print a full array)
    // {0}, {1} ... is used to identify which parameters are used to print
    // {} is a placeholder that is the default.
    // println!("You guessed: {}", guess);

    //* match : is like switch case
    //* cmp : is method compares two values and can be called on anything that can be compared. It takes a reference to whatever you want to compare with
    match the_guessed_number.cmp(&the_secret_number) {
      Ordering::Less => println!("Too Low"),
      Ordering::Greater => println!("Too High"),
      Ordering::Equal => {
        println!("Correct, You Win!");
        break;
      }
    }
  }
}

mod testing{
  #[test]
  fn check_if_bigger_number_will_give_greater(){

  }
}