#![allow(unused_imports, unused_variables)]

use std::fs;
use std::fs::File;
use std::io::{self, Read};
use std::io::{ErrorKind, Write};

pub fn run() {
    // let mut f = match File::open("hello.txt") {
    //     Ok(file) => file,
    //     Err(e) => match e.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(file) => file,
    //             Err(e) => panic!("Error Creating the file: {}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error)
    //         }
    //     },
    // };

    // OR CAN BE WRITTEN USING CLOSURES
    // let f = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });

    // unwrap
    // If the Result value is the Ok variant, unwrap will return the value inside the Ok.
    // If the Result is the Err variant, unwrap will call the panic! macro for us (predetermined error massages).
    let f = File::open("hello.txt").unwrap();

    // expect
    // If the Result value is the Ok variant, unwrap will return the value inside the Ok.
    // If the Result is the Err variant, unwrap will call the panic! macro with a msg that we provide as a parameter to the expect function
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

// Propagating Errors
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// can be rewritten with the ? operation
// The ? operator can be used in functions that have a return type of Result, because it is defined to work in the same way as the match expression
// we’re only allowed to use the ? operator in a function that returns Result or Option or another type that implements std::ops::Try
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// a much shorter way
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// even shorter
fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
