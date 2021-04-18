#![allow(dead_code, unused_variables, unused_mut, unused_assignments)]
use std::ops::{Add, AddAssign}; // import the Add trait

pub fn run() {
    let mut c1 = Complex::new(10, 20);
    let mut c2 = Complex::new(30, 20);
    let mut c3 = Complex::new(10, 20);
    let mut c4 = Complex::new(30, 20);
    let mut c5 = Complex::new(30, 20);
    let mut c6: Complex<i32>;
    // test the Add trait
    println!("c1 + c2 = {:?}", c1 + c2);

    // test the AddAssign trait
    c4 += c3;
    println!("c4 += c3 >> c4 = {:?}", c4);

    // test the Clone trait
    c6 = c5; // move value
             //println!("c6 =  {:?}", c5); // this will give an error due to value moved in the previous line
    c5 = c6.clone();
    println!("c5 =  {:?}", c5); // this will work due to only cloning the value in the previous line

    // test the partial equality
    println!("c5 == c6 in boolean is {}", c5 == c6);
}

// Define a generic struct that takes T
#[derive(Debug)]
struct Complex<T> {
    real: T,
    imaj: T,
}

// implement the struct methods over any datatype T
impl<T> Complex<T> {
    fn new(real: T, imaj: T) -> Self {
        Complex::<T> { real, imaj } // here we return the Complex type of any entered value in real and imaj
    }
}

// Implement the Add(+) trait for the Complex type
impl<T> Add for Complex<T>
where
    T: Add<Output = T>,
{
    type Output = Complex<T>; // this is an associated type >> you must specify the output datatype that this operation will result to
    fn add(self, rhs: Self) -> Self::Output {
        Complex::<T> {
            real: self.real + rhs.real,
            imaj: self.imaj + rhs.imaj,
        }
    }
}

// Implement the AddAssign(+=) trait for the Complex type
impl<T> AddAssign for Complex<T>
where
    T: AddAssign<T>,
{
    fn add_assign(&mut self, rhs: Self) {
        self.real += rhs.real;
        self.imaj += rhs.imaj;
    }
}

// Implement the Clone trait for the Complex type
impl<T> Clone for Complex<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            real: self.real.clone(),
            imaj: self.imaj.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.real = source.real.clone();
        self.imaj = source.imaj.clone();
    }
}

// Implement the PartialEq trait for the Complex type
impl<T> PartialEq for Complex<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.real == other.real && self.imaj == other.imaj
    }
}

// Implement the Eq trait for the Complex type >> it's directly derived form the PartialEq trait
impl<T> Eq for Complex<T> where T: Eq {}
