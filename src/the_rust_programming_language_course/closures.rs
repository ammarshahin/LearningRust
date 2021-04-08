pub fn run() {
    let fun = say_hello; // here the fun variable is now of the fn() data type meaning it's an alias of the function say_hello
    fun(); // invoke the function say_hello with the alias fun variable

    // Define a closure is by the following
    // let func_name = |input_parameter_with_annotations| -> return_type {function_body + the_return_expression}
    // Note: the return type is optional >> the compiler can do it for you >> BAD PRACTICE
    let add = |x: i32, y: i32| -> i32 { x + y };
    println!("20 + 30 = {res}", res = add(20, 30)); // here we invoke the add closure function such as any other function

    // The closures can borrow any variable from the scope it was declared in
    let x = 5;
    let c = || x + 1; // here the closure can access the variable x
    let y = c();
    println!("y = {}", y);

    println!("{}", func(12));
}

fn say_hello() {
    println!("hello!!");
}

// Important Note: when passing a value to a function note that you need to declare it's parameters as any ordinary variable but without using the let keyword
// in this example
// fn func(x: i32) -> i32 { // >> this will not work because we declared the parameter x as an immutable i32 variable while in the body of the function we actually try to modify it
// this is the correct way as the x need to be mutable to be able to modify it
fn func(mut x: i32) -> i32 {
    x = x + 12;
    x
}
