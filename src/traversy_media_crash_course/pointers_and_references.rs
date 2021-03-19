//* reference pointer >> Point to a resource in memory

pub fn run() {
    //* with primitive types
    let arr1 = [1, 2, 3];
    let mut arr2 = arr1; // arr2 now just a copy of arr1
    arr2[2] = 10; // here we are mutating the arr2 only >> arr1 is still the same
    println!("values : {:?}", (arr1, arr2));

    //* with non-preemptive types, if you assign another variable to a piece of data, the first(original) variable will no longer hold that value. You'll need to use a reference (&) to point to the resource

    //* vector
    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;
    // println!("values : {:?}", (vec1, vec2)); // this will give an error
    println!("values : {:?}", (&vec1, vec2));
}
