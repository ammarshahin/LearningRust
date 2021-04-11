trait Printable {
    fn format(&self) -> String;
}

impl Printable for i32 {
    fn format(&self) -> String {
        format!("i32 : {}", self)
    }
}

impl Printable for String {
    fn format(&self) -> String {
        format!("String: {}", self)
    }
}

pub fn run() {
    let a = 20;
    let b = "hello".to_string();

    // static approach
    println!("{}", a.format());
    println!("{}", b.format());

    // Static Dispatch
    print_it(a);
    print_it(b);
}

// Static Dispatch
// here with every call the compiler create a function that is suited to the passed variable
fn print_it<T: Printable>(s: T) {
    println!("{}", s.format());
}
