use std::mem;

const VAR: u8 = 20; // no fixed address >> (Text replacement)
static VAR_STATIC_IMMUT: u8 = 50; // HAS a fixed address
static mut VAR_STATIC_MUT: u8 = 50; // declare a global variable that can be mutated any where >> BAD!!!! can be used as unsafe set of rust

pub fn run() {
    // Creating an uninitialized variable
    let x: i32; // x is uninitialized and cannot be used without assigning a value to it

    let b = 1321321;
    println!("the size of {} is {}", b, mem::size_of_val(&b));
    let x = i32::pow(3, 2);
    println!("{}", x);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3); // get the power of an integer number float^integer
    let b_to_pi = f64::powf(b, 3.1); // get the power of an float number float^float
    println!("{} {} {}", b, b_cubed, b_to_pi);
    let b_to_pi = f64::powf(b, std::f64::consts::PI); // get the power of an float number float^float (here we got the compiler saved value of pi)
    println!("{} {} {}", b, b_cubed, b_to_pi);

    // bitwise operations
    let mut c = 1 | 2;
    println!("(1 | 2) = {}", c);
    c = 1 & 3;
    println!("(1 & 3) = {}", c);
    // Shift operation
    c = 1 << 10;
    println!("(1 << 10) == (2^10) = {}", c);
    // logical
    // here we will redeclare the variable c again to change it's data type (Shadowing the old variable with the new one)
    let c = std::f64::consts::PI < 4.0; // any logical comparison will return a boolean
    println!("(PI < 4) = {}", c);
    let c = 5;
    let c = c == 55; // we used the same variable twice with different data type for both
    println!("(5 == 55) = {}", c);

    // scope and shadowing
    let a = 123;
    println!("a = {}", a);
    let a = 777;
    println!("a = {}", a);
    {
        let a = 999;
        println!("a = {}", a);
    }
    println!("a = {}", a);

    // allocating variable in the heap
    // Box<anyType> = Box::new(initialization to the variable);
    let var: Box<u8> = Box::new(50); // create a dynamically allocated variable var u8 with Box
    println!("var = {}", &var);
}
