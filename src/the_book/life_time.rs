// Lifetime Elision : A set of rules that allow us to skip having to provide a life time for the function signature and the compiler doit for us
//
// The first rule is that each parameter that is a reference gets its own lifetime parameter.
// In other words, a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32);
// a function with two parameters gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.
//
// The second rule is if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.
//
// The third rule is if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method,
// the lifetime of self is assigned to all output lifetime parameters.
// This third rule makes methods much nicer to read and write because fewer symbols are necessary.

pub fn run() {
    let s1 = String::from("Hello");

    let res;

    {
        let s2 = String::from("World !");
        res = longest(&s1, &s2);
        println!("the longest line is {}", res); // will work because the res has a life time equal to the smaller string(s2)
    }

    // println!("the longest line is {}", res); // will not work because the res has a bigger life time than the smaller string(s2)
}

// here we specified that the return has at least a life time equal to the smallest life time of the input parameters
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
