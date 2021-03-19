pub fn run() {
    //*  Call and pass strings
    say_greetings("hello", "ammar");

    //* Call and pass numbers and return the sum
    println!("the sum of 10, 20 is {}", add(10, 20));

    //* Closure (Like lambda function) >> (One line functions)
    //* the scope of the closure function is it's own scope + the scope of the function containing it
    let n3 = 5;
    //*  variable   = |the_passed_argus| the_implementation_and_the_return_value;
    let add_numbers = |n1: i32, n2: i32| n1 + n2 + n3;

    println!(
        "the closure sum of 5, 15, and {} is {}",
        n3,
        add_numbers(5, 15)
    );
}

fn say_greetings(greet: &str, name: &str) {
    println!("{} {}", greet, name);
}

//* function with return type
//* fn function_name(the_passed_argus) -> return_data_type
fn add(n1: i32, n2: i32) -> i32 {
    //* return value
    return n1 + n2;

    //* another way (without semicolon)
    //n1 + n2
}
