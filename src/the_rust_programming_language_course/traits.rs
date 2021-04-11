use std::fmt::Debug;

pub fn run() {
    // Intro
    let h = Human::new("Adam"); // Same as >>     Human { name: "Adam" };
    h.talk();

    let c = Cat::new("Bosy"); // Same as >>          Cat { name: "Bosy" };
    c.talk();

    // Generic traits
    let a: Vec<i32> = vec![1, 2, 3];
    println!("sum = {}", a.sum());
    let a: Vec<u32> = vec![1, 2, 7];
    println!("sum = {}", a.sum());
    let a: [i32; 3] = [1, 7, 7];
    println!("sum = {}", a.sum());

    //Trait parameter
    let cir = Circle { radius: 10.0 };
    //println!("Area = {}", cir.area());
    print_info(cir);
    let sq = Square { side: 20.0 };
    //println!("Area = {}", sq.area());
    print_info(sq);
}

/***************************************************************************************************/
/***************************************** Intro ***************************************************/
/***************************************************************************************************/
trait Animal {
    fn name(&self) -> &'static str; // no default implementation for this method so we must provide one for every struct that implement the trait Animal

    // Default implementation was provided for this method so we don't have to provide one for every struct that implement the trait Animal
    // We can provide a special implementation for every struct to override the default
    fn talk(&self) {
        // this is default method
        println!("{} cannot talk.!!", self.name())
    }

    fn new(name: &'static str) -> Self;
}
struct Human {
    name: &'static str,
}
struct Cat {
    name: &'static str,
}
impl Animal for Human {
    fn name(&self) -> &'static str {
        self.name
    }

    // here we provided an implementation for the talk method so it will override the default method and will be executed instead
    fn talk(&self) {
        println!("{} can talk.!!", self.name())
    }

    fn new(name: &'static str) -> Self {
        Self { name }
    }
}
impl Animal for Cat {
    fn name(&self) -> &'static str {
        self.name
    }
    // here we didn't provide an implementation for the talk method so the default will be executed

    fn new(name: &'static str) -> Self {
        Self { name }
    }
}

/***************************************************************************************************/
/**********************************Generic traits***************************************************/
/***************************************************************************************************/
// define a generic trait for any collection of type T
trait Summable<T> {
    fn sum(&self) -> T;
}
// implement the trait methods for the Vec<i32>
impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut sum = 0;
        for i in self.iter() {
            sum += i;
        }
        sum
    }
}
// implement the trait methods for the Vec<u32>
impl Summable<u32> for Vec<u32> {
    fn sum(&self) -> u32 {
        let mut sum = 0;
        for i in self.iter() {
            sum += i;
        }
        sum
    }
}
// implement the trait methods for the [i32] "Array of i32"
impl Summable<i32> for [i32] {
    fn sum(&self) -> i32 {
        let mut sum = 0;
        for i in self.iter() {
            sum += i;
        }
        sum
    }
}

/***************************************************************************************************/
/**********************************Trait parameter**************************************************/
/***************************************************************************************************/
trait Shape {
    fn area(&self) -> f64;
}

#[derive(Debug)]
struct Square {
    side: f32,
}

#[derive(Debug)]
struct Circle {
    radius: f32,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        (self.side * self.side) as f64
    }
}
impl Shape for Circle {
    fn area(&self) -> f64 {
        ((self.radius * self.radius) as f64 * std::f64::consts::PI) as f64
    }
}

// print_info   is a generic function that takes any type that implements the Shape trait and the Debug trait
// This is like polymorphism
// you can add any number of traits using the + operator

// fn print_info(shape: impl Shape + Debug) {
// Same as {trait bound syntax}
// fn print_info<T: Shape + Debug>(shape: T) {
// Same as
fn print_info<T>(shape: T)
where
    T: Shape + Debug,
{
    println!("{:?}", shape);
    println!("Area = {}", shape.area());
}
