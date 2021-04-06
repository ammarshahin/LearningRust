// Tuples group together values of different types
// Max 12 elements

pub fn run() {
    // Define a tuple >> let tuple_name: (tuple_element0_type, tuple_element1_type, tuple_element2_type) = (tuple_element0, tuple_element1, tuple_element2)
    let person: (&str, &str, u8) = ("Ammar", "Egypt", 26);

    println!(
        "{} is from {} and his age is {}",
        person.0, person.1, person.2
    );

    let blue: (u8, u8, u8) = (20, 20, 20); // is equivalent to >> let blue = (20, 20, 20); // no type annotation required
    let (x, y, z) = blue; // destruct the tuple
    println!("Color : ({}, {}, {})", x, y, z);
}
