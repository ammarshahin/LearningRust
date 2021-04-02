//* The Rules of References
//*     - At any given time, you can have either one mutable reference or any number of immutable references.
//*     - References must always be valid.

pub fn run() {
    let mut s1 = String::from("hello");

    let len = calculate_length(&s1); // pass by refrence

    println!("The length of '{}' is {}.", s1, len);

    change(&mut s1);
    println!("the string after change : {}", s1);
}

//* this function takes a ref to string and returns usize integer
fn calculate_length(s: &String) -> usize {
    // s is a reference to a String (borrowing) .. but it cannot change it or modify it.
    return s.len();
} // Here, s goes out of scope. But because it does not have ownership of what it refers to, nothing happens.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
