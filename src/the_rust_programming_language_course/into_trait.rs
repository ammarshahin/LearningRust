// Into trait is a trait that is implemented to any convertible types
// ex: Into<String>      >> means any datatype that con be converted to String

pub fn run() {
    let p1 = Person::new("AmmarShahin");
    let p2 = Person::new("ammar M.shahin".to_string());
    println!("name = {}", p1.get_name());
    println!("name = {}", p2.get_name());
    // Now both passing String and passing a &str works normally and the method takes both without any problem
}

struct Person {
    name: String,
}

impl Person {
    // fn new(name: &str) -> Self {   // OLD way
    // here we are saying that convert the type that is passed to you to String before using it
    // fn new<S: Into<String>>(name: S) -> Self {
    // Same as
    fn new<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        Person { name: name.into() }
    }

    fn get_name(&self) -> &str {
        &self.name // return a reference to the String
    }
}
