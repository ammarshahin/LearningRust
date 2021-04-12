// By default the compiler adds Lifetime Elision to all of the methods of any struct so that it knows when the method will end

#[derive(Debug)]
struct Person {
    name: String,
}

// here we added a life time specifier to till the compiler that this reference will live as long the object lives
#[derive(Debug)]
struct Company<'l> {
    name: String,
    ceo: &'l Person,
}

struct Human<'b> {
    name: &'b str,
}

impl<'b> Human<'b> {
    fn talk(&self) {
        println!("Hi, my name is {}", self.name);
    }
}

pub fn run() {
    let boss = Person {
        name: String::from("Elon"),
    };
    let com = Company {
        name: String::from("Tesla"),
        ceo: &boss,
    };

    println!("{:#?}", com);

    let h = Human { name: "John" };
    h.talk();
}
