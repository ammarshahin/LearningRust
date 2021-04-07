#![allow(dead_code, unused_variables, unused_mut)]

pub fn run() {
    let mut vec = vec![1, 2, 3];
    // Note that here the elements of the vector has been moved to x and not borrowed >> so after the loop is ended the vector is no longer available
    for i in vec {
        print!("{}, ", i)
    }
    // vec[2] = 5; // this will give an error !!!! >>  borrow of moved value: `vec`

    // So the approach is to use borrow instead of move
    let mut vec = vec![1, 2, 3];
    for i in &vec {
        print!("{}, ", i);
    }
    vec[2] = 5; // this now works

    println!();

    // The best way to iterate over a collection of data is using the iter method
    for (index, elem) in vec.iter().enumerate() {
        print!("vec[{}] = {}, ", index, elem);
    }

    println!();

    // but the previous example only returns immutable reference to the elements
    for (index, elem) in vec.iter_mut().enumerate() {
        *elem *= 2;
        print!("vec[{}] = {}, ", index, elem);
    }

    println!();

    println!("vec = {:?}, ", vec);
}

// Other methods related to iter >>
//      - .rev() >> to iterate throw the collection with a reverse order
