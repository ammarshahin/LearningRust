#![allow(dead_code, unused_variables)]

pub fn run() {
    let mut a = [10; 5];
    let b = &a[0..3]; // now b has a slice of >> has borrowed part of it
    println!("b = {:?}", b);

    println!("the sum of the array = {}", use_slice(&a)); // pass to a function as an immutable reference

    change_slice(&mut a); // pass to a function as an mutable reference
    println!("new a = {:?}", a);
}

fn use_slice(s: &[i32]) -> i32 {
    let mut sum = 0;
    for elem in s {
        sum += *elem;
    }
    sum
}

fn change_slice(s: &mut [i32]) {
    for elem in s {
        *elem = 0;
    }
}
