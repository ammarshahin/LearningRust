// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_mut, unused_variables)]

//* if and loops take only boolean expressions as conditions >> no integers

pub fn run() {
    const CONDITION: bool = true;
    let x = if CONDITION { 5 } else { 6 }; //* if is an expression not a statement >> no semicolon then no return needed
    println!("x = {}", x);

    'b: loop {
        loop {
            // break; //* break the most inner loop
            break 'b; //* break the labeled loop
        }
    }

    'c: loop {
        loop {
            // continue; //* continue the most inner loop
            // continue 'c; //* continue the labeled loop
            break 'c;
        }
    }

    while CONDITION {
        println!("while inside");
        break;
    }

    for parm in [7, 66, 25].iter() {
        println!("parm = {}", parm);
    }

    //* loop from 0 to 4
    for num in 0..5 {
        println!("num = {}", num);
    }

    //* loop from 0 to 5
    for num in 0..=5 {
        println!("num = {}", num);
    }

    // This collects any command-line arguments into a vector of Strings.
    // For example:
    //
    //     cargo run apple banana
    //
    // ...produces the equivalent of
    //
    //     vec!["apple".to_string(), "banana".to_string()]
    let args: Vec<String> = std::env::args().skip(1).collect();

    // This consumes the `args` vector to iterate through each String
    for arg in args {
        // 1a. Your task: handle the command-line arguments!
        //
        // - If arg is "sum", then call the sum() function
        // - If arg is "double", then call the double() function
        // - If arg is anything else, then call the count() function, passing "arg" to it.
        if arg == "sum" {
            sum();
        } else if arg == "double" {
            double();
        } else {
            count(arg);
        }
        // 1b. Now try passing "sum", "double" and "bananas" to the program by adding your argument
        // after "cargo run".  For example "cargo run sum"
    }
}

fn sum() {
    let mut sum = 0;
    // 2. Use a "for loop" to iterate through integers from 7 to 23 *inclusive* using a range
    // and add them all together (increment the `sum` variable).  Hint: You should get 255
    // Run it with `cargo run sum`
    for num in 7..=23 {
        sum += num;
    }
    println!("The sum is {}", sum);
}

fn double() {
    let mut count = 0;
    let mut x = 1;
    // 3. Use a "while loop" to count how many times you can double the value of `x` (multiply `x`
    // by 2) before it is larger than 500.  Increment `count` each time through the loop. Run it
    // with `cargo run double`  Hint: The answer is 9 times.

    while x < 500 {
        x *= 2;
        count += 1;
    }

    println!(
        "You can double x {} times before it is larger than 500",
        count
    );
}

fn count(arg: String) {
    // Challenge: Use an unconditional loop (`loop`) to print `arg` 8 times, and then break.
    // You will need to count your loops, somehow.  Run it with `cargo run bananas`
    //
    let mut count = 0;
    loop {
        print!("{} ", arg); // Execute this line 8 times, and then break. `print!` doesn't add a newline.
        count += 1;
        if count >= 8 {
            break;
        }
    }

    println!(); // This will output just a newline at the end for cleanliness.
}
