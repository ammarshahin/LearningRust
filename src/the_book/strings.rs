#![allow(dead_code, unused_variables)]

pub fn run() {
    let mut empty_string = String::new();
    let init_string1 = String::from("init String");
    let init_string2 = "init String".to_string(); // to_string === to_owned

    // append a single char to the string
    empty_string.push('a');
    println!("s = {}", empty_string);

    // clear a String
    empty_string.clear();
    println!("s = {}", empty_string);

    // append a full string to the end of another string
    empty_string.push_str("Hello");
    println!("s = {}", empty_string);
    empty_string.push_str(" World !!");
    println!("s = {}", empty_string);

    //* Concatenation of strings
    // the add overload operator in String consumes the first string and takes a reference to the second one
    let s1 = "hello".to_string();
    let s2 = " Rust!!".to_string();
    let s3 = s1 + &s2; // {s1 is now moved and will be dropped after the operation is complete}
                       // println!("s = {}", s1); // will give an error
    println!("s3 = {}", s3);

    // A Better way to concatenation
    let s4 = format!("{0}- {0}- {0}- {0}", s3);
    println!("s4 = {}", s4);

    // Slicing Strings
    let s5 = &s4[0..5];
    println!("s5 = {}", s5);
}
