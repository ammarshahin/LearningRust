pub fn run() {
    // x is immutable variable
    let x = 5;
    // x is now a shadow variable >> By using let (shadowing) , we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.
    //* Note: Why? >> because we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name.
    let x = x + 1;
    // y is mutable variable
    let mut y = 5;
    y = y + 1;
    // Z is a constant variable
    const Z: u8 = 20; // always immutable, must be annotated, must be set to a constant expression, not the result of a function call or any other value that could only be computed at runtime, cannot be shadowed.
    y = y + Z;
    println!("x = {}, y = {}", x, y);

    println!("********************************");
    //* A tuple is a general way of grouping together a number of values with a variety of types into one compound type.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (xx, yy, zz) = tup; //* pattern matching to destructure a tuple value (destructuring)
    println!("tup = {}, {}, {}", xx, yy, zz);
    //* is equivalent to
    println!("tup = {}, {}, {}", tup.0, tup.1, tup.2);

    loops();
}

fn loops() {
    //* Returning Values from infinite Loops
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter >= 10 {
            break counter * 2;
        }
    };

    println!("result = {}", result);

    println!("\n********************************while loop********************************");

    //* while loop
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    println!("\n********************************for loop********************************");

    //* for loop
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    //* for range loop
    for element in 1..4 {
        println!("the value is: {}", element); // will print 1 , 2 , 3
    }
}
