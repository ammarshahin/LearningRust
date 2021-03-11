// Arrays are fixed lists where elements are of the same data types

pub fn run() {
    // Define array >> let array_name: [data_type; number of elements] = [the elements];
    let numbers: [u8; 5] = [1, 2, 3, 4, 5];
    // // Define array >> simpler form
    // let arr = [1, 2, 3, 4, 5];

    println!("The array contents are: {:?}", numbers);

    // print the array
    for index in 0..numbers.len() {
        println!("array[{}] = {}", index, numbers[index]);
    }

    let mut arr: [u8; 5] = [10, 20, 30, 40, 50];

    // fill and print the array
    for index in 0..arr.len() {
        arr[index] *= 2;
        println!("arr[{}] = {}", index, arr[index]);
    }
}
