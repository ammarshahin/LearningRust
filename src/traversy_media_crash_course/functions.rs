pub fn run() {
    // pass strings
    say_greetings("hello", "ammar");

    // pass numbers and return
    println!("the sum of 10, 20 is = {}", add(10, 20));

    // Closure (Like lambda function)
    let add_nums = |n1: i32, n2: i32| n1 + n2;
    println!("the closure sum of 5, 15 is = {}", add_nums(5, 15));
}

fn say_greetings(greet: &str, name: &str) {
    println!("{} {}", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    // return value
    // return n1 + n2;

    // another way (without semicolon)
    n1 + n2
}
