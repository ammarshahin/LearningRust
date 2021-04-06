#![allow(dead_code, unused_variables, unreachable_patterns)]

pub fn run() {
    //structs();
    enums();
}

enum Color {
    Red,
    Blue,
    Green,
    RgpColor(u8, u8, u8), // define a field that takes several arguments (like tuple)
    Cmyk { yellow: u8, gray: u8, black: u8 },
}

fn enums() {
    let c1 = Color::Red;
    let c2 = Color::RgpColor(50, 80, 99);
    let c3 = Color::Cmyk {
        yellow: 50,
        gray: 60,
        black: 255,
    };

    let c = c3;
    match c {
        Color::Blue => println!("BLUE"),
        Color::Red => println!("RED"),
        Color::Green => println!("Green"),
        Color::RgpColor(0, 0, 0)
        // OR condition inside the match
        | Color::Cmyk {
            yellow: _, // any value of yellow
            gray: _, // any value of gray
            black: 255,
        } => println!("BLACK"), // handle a special value of Rgb color
        Color::RgpColor(r, g, b) => println!("An RGB color with ({} ,{} ,{})", r, g, b), // handle any other value of Rgb color
        Color::Cmyk {
            yellow: 50,
            gray: 60,
            black: 80,
        } => println!("Cmyk color"), // handle a special value of Cmyk color
        Color::Cmyk {
            yellow: _,     // any value of yellow
            gray: _,       // any value of gray
            black: 0..=50, // any black value between 0 and 50
        } => println!("An Cmyk color"), // handle any value of Cmyk color
        _ => println!("other color !!!"), // handle any other condition (default case)
    }
}

struct Point {
    x: f64,
    y: f64,
}

struct Line {
    start: Point,
    end: Point,
}

fn structs() {
    let p1 = Point { x: 3.4, y: 4.0 };
    let p2 = Point { x: 5.0, y: 10.0 };
    let line = Line { start: p1, end: p2 };
    println!(
        "line ({}, {}) and ({},{})",
        line.start.x, line.start.y, line.end.x, line.end.y
    );
}
