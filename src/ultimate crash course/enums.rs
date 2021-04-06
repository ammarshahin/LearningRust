enum Color {
    Red,
    Blue = 10,
    Green,
}

pub fn run() {
    let e = Color::Blue; // e now has the variant value of Blue
                         // println!("e = {:?}", e); // Error Can't be printed
}
