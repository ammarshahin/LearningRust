#![allow(dead_code, unused_variables)]

use std::fmt::Debug;
use std::ops::Add;

pub fn run() {
    generics_study();
}

fn generics_study() {
    // Generic Function
    // assume we have a list that we need to find the largest number in it
    let list1: Vec<i32> = vec![1, 56, 121, 154, 24354];
    let list2: Vec<u8> = vec![1, 56, 34, 12, 15, 24];
    let list3: Vec<f64> = vec![1.0, 56.0, 34545343.5, 121.0, 154.0, 24354.0];
    let largest = largest_number(&list3);
    println!("the largest number in {:?} is = {}", list3, largest);

    // Generic Struct
    let p1 = Point::new(1.5, 2.5);
    let p2 = Point::new(5.5, 7.5);
    let l = Line::new(p1, p2);
    println!("len = {}", l.len());
}

// Generic Struct
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct Line<T> {
    p1: Point<T>,
    p2: Point<T>,
}

// implement some methods that are only applicable over T = f64  >> will not work for  any other type
impl Line<f64> {
    fn len(&self) -> f64 {
        (((self.p1.x + self.p2.x) as f64).powf(2.0) + (((self.p1.y + self.p2.y) as f64).powf(2.0)))
            .sqrt()
    }
}

impl<T> Line<T>
where
    T: Add + Copy,
{
    fn new(p1: Point<T>, p2: Point<T>) -> Self {
        Line { p1, p2 }
    }
}

impl<T> Point<T>
where
    T: Add + Copy,
{
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

// Generic Function
fn largest_number<T>(list: &[T]) -> &T
where
    T: PartialOrd, // T has to implement the PartialOrd trait so that we can compare them and determine the largest (to use the < > = compressions)
{
    let mut largest = &list[0];
    for x in list {
        if x > largest {
            largest = x;
        }
    }
    largest
}
