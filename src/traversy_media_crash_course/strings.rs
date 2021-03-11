// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
    let greetings1 = "hello from the stack"; // Primitive str
    let mut greetings2 = String::from("hello from the heap"); // String Class
    println!("The primitive str is {}", greetings1);
    println!("String class str is {}", greetings2);

    // get length
    println!("Length is: {}", greetings2.len()); // len isd the actual data memory used in this string
                                                 // get Capacity in Bytes
    println!("Capacity is: {}", greetings2.capacity()); // capacity is the amount of memory allocated to this particular object
                                                        // get if the string is empty
    println!("str is_empty: {}", greetings2.is_empty());
    // check if it contains a sub string
    println!("str contains hello: {}", greetings2.contains("hello"));
    // Replace a sub string with another sub string
    println!(
        "str contains hello: {}",
        greetings2.replace("hello", "greetings")
    );

    // modify the memory allocated string
    greetings2.push(' '); // push a single character
    greetings2.push_str("World"); // push a full string
    println!("String class str after modification is: {}", greetings2);

    // loop through string with wight spaces
    for word in greetings2.split_whitespace() {
        println!("{}", word);
    }

    // create a string with a certain capacity
    let mut greetings3 = String::with_capacity(10);
    greetings3.push_str("ammar"); // dynamic allocation lets you allocate more memory to fit more char's
    println!("{}", greetings3);

    // assertion testing
    assert_eq!(greetings3.capacity(), 10); // pass
    assert_eq!(greetings3.len(), 8); // Error >> assertion failed: `(left == right)` left: `5`, right: `8`', src\strings.rs:40:5

    
}
