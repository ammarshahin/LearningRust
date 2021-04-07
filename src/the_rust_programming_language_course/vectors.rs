#![allow(dead_code, unused_variables)]

pub fn run() {
    let mut v1: Vec<u8> = Vec::new(); // create an empty vector using the explicit declaration
    let v2 = vec![1, 2, 3]; // create a vector using the implicit declaration
    println!("v = {:?}", v1); // will print out an empty vector
    for i in 0..10 {
        v1.push(i); // push values to the vector
    }
    println!("v = {:?}", v1); // will print out the vector elements

    // Pop the last element of the vector
    v1.pop();
    println!("v = {:?}", v1); // will print out the vector elements

    // Get single value
    println!("Single Value: {}", v1[0]);
    // note that this type of access is not safe as we are able to access out of boundaries indexes
    // this is solved by using the .get(index) method .. it returns option<T> value to check whether the access is valid or not
    let valid_access = match v1.get(2) {
        Some(&i) => i,
        None => {
            // Error handling
            println!("invalid access !!!!");
            0
        }
    };

    println!("v[20] = {}", valid_access);

    // Get vector length
    println!("Vector Length: {}", v1.len());

    // Vectors are heap allocated
    println!("Vector occupies {} bytes", std::mem::size_of_val(&v1));

    // Get Slice
    let slice: &[u8] = &v1[1..3];
    println!("Slice: {:?}", slice);

    // Loop through vector values
    // note that i is an usize variable
    //
    for (i, x) in v1.iter().enumerate() {
        print!("v[{}] = {}  ", i, x);
    }

    println!(); // print an empty new line

    // Loop & mutate values >> iter_mut returns an mut ref to the value
    for x in v1.iter_mut() {
        *x *= 2;
    }
    println!("v = {:?}", v1); // will print out the vector elements

    // pop all of the elements from the vector safely
    // this loop will pop all of the elements one by one till the .pop method returns None and the loop exits
    while let Some(x) = v1.pop() {
        println!("{}", x);
    }

    // into iterator >> is a way of moving all of the elements of a collection and move it to an into_iter variable which make the original collection no longer available
    // this is useful when we don't care about a collection any more and we only intersted in the elements it self
    // let into_iter = v2.into_iter(); // the .into_iter() method consumes the vector itself and returns an intoiter<T> variable that contains all of the elements
    // v1.extend(into_iter); // now the into_iter is consumed and appended to the end of the v1 vector
    // the previous two lines can be comprised to one line
    v1.extend(v2);

    println!("v1 = {:?}", v1); // will print out the vector elements
}
