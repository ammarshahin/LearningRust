//! Arrays are fixed lists where elements are of the same data types

pub fn run() {
    //* Define array >> let array_name: [data_type; number of elements] = [the elements];
    let _arr0: [u8; 5] = [1, 2, 3, 4, 5];

    //* Initializing arrays

    //* 1 - fill all of the array
    let _arr1 = [1, 2, 3, 4, 5]; // equivalent to let _arr1: [i32; 5] = [1, 2, 3, 4, 5];

    //* 2 - fill with the same value
    let _arr2 = [0; 5]; // all 5 elements values are set to 0

    //* 3 - make mutable array
    let mut _arr3 = [0; 5]; // all 5 elements values are set to 0

    //*  fill the array
    for index in 0.._arr3.len() {
        _arr3[index] = index * 2;
    }

    //* print the array
    for index in 0.._arr3.len() {
        println!("array[{}] = {}", index, _arr3[index]);
    }
    //* Or
    // println!("The array contents are: {:?}", numbers);
}
