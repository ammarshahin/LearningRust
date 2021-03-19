//! Struct : used to create custom data types

//* Traditional Struct
struct Color {
    red: u8,
    blue: u8,
    green: u8,
}

//* Tuple Struct (no need to name the inner elements)
struct ColorRBG(u8, u8, u8);

pub fn run() {
    //* Define(Construct) an object of the traditional struct
    let mut c1 = Color {
        red: 200,
        blue: 200,
        green: 200,
    };

    //* mutate the struct elements
    //* Access the struct using inner elements names
    c1.blue = 250;

    println!("Color : {} {} {}", c1.red, c1.blue, c1.green);

    //********************************************************************/
    //* Define(Construct) an object of the tuple struct
    let mut c2 = ColorRBG(200, 0, 0);

    //* Access the struct using the index
    c2.0 = 0;
    println!("Color : {} {} {}", c2.0, c2.1, c2.2);
}
