#![allow(dead_code, unused_variables, unreachable_patterns)]

pub fn run() {
  structs();
  // enums();
  // option();
  // generic_structs();
}

// T is anytype that can construct the struct >> will be replaced with the passed value type
#[derive(Debug)]
struct GenericPoint<T> {
  x: T,
  y: T,
}

fn generic_structs() {
  let a = GenericPoint { x: 1, y: 2 }; // T is now i32
  let b = GenericPoint { x: 1.1, y: 2.5 }; // T is now f64
  let c: GenericPoint<u8> = GenericPoint { x: 6, y: 12 }; // Here we annotate the value ourselves >>  T is now u8
  println!("a = {:?}", a);
  println!("b = {:?}", b);
  println!("c = {:?}", c);
}

fn option() {
  let x = 50;
  let y = 2;
  let result = if y == 0 { None } else { Some(x / y) };
  match result {
    Some(i) => println!("{} / {} = {}", x, y, i),
    None => println!("Cannot divide by zero!!!"),
  }

  // the if let keyword
  // The let statement will allow you to check whether or not what's on the right hand side can in fact be
  // assigned to what's on the left and if it can be assigned then the evaluation of the later statement will give true.
  //
  // Some(_) meaning for any valid value that Some can hold
  if let Some(_) = result {
    println!("{} / {} = {}", x, y, result.unwrap()); // unwrap() >> Returns the contained [`Some`] value or a provided default.
  } else {
    println!("{} / {} = {:?}", x, y, result.unwrap_or_else(|| 0)); // unwrap_or_else() >> Returns the contained [`Some`] value or computes it from a closure.
  }
}

enum Color {
  Red,
  Blue,
  Green,
  RgpColor(u8, u8, u8), // define a field that takes several arguments (like tuple)
  Cmyk { yellow: u8, gray: u8, black: u8 }, // define a field that takes several arguments (Struct)
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
    // OR condition inside the match
    // handle a special value of Rgb color. Or we can use the .. after for the wanted variant to indicate that we only care for it
    // | Color::Cmyk { yellow: _, gray: _, black: 255} => println!("BLACK"),
    Color::RgpColor(0, 0, 0) | Color::Cmyk { black: 255, .. } => println!("BLACK"), // are the same
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
