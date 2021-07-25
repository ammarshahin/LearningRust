/************************** traits *********************************/
trait Area {
    fn get_area(&self) -> u8;
}

// generic functions that takes a generic type T
fn shape_get_area<T>(item: T)
where
    T: Area,
{
    println!("the area = {}", item.get_area());
}

/************************** structs *********************************/
/* 1- Rectangle ********************************/
struct Rectangle {
    width: u8,
    height: u8,
}

impl Rectangle {
    fn new(w: u8, h: u8) -> Self {
        Self {
            width: w,
            height: h,
        }
    }

    //* Class getters
    fn get_width(&self) -> u8 {
        return self.width;
    }

    fn get_height(&self) -> u8 {
        return self.height;
    }

    //* Class setters
    fn change_height(&mut self, h: u8) {
        self.height = h;
    }

    fn change_width(&mut self, w: u8) {
        self.width = w;
    }
}

// trait implementation for the Struct Rectangle
impl Area for Rectangle {
    fn get_area(&self) -> u8 {
        return self.height * self.width;
    }
}

/* 2- Triangle ********************************/
struct Triangle {
    width: u8,
    height: u8,
}

impl Triangle {
    fn new(w: u8, h: u8) -> Self {
        Self {
            width: w,
            height: h,
        }
    }
}

impl Area for Triangle {
    fn get_area(&self) -> u8 {
        (0.5_f32 * (self.height as f32) * (self.width as f32)) as u8 // to convert from type to another
    }
}

pub fn run() {
    // create an object
    //* using the explicit constructor
    let rec_one = Rectangle::new(6, 4);

    // access the struct elements
    println!(
        "area of the rectangle {} * {} = {}",
        rec_one.get_width(),
        rec_one.get_height(),
        rec_one.get_area()
    );

    // using the implicit constructor
    let mut rec_two = Rectangle {
        height: 6,
        width: 4,
    };
    //* change the internal attributes using the setters
    rec_two.change_width(8);
    rec_two.change_height(8);

    // using the Area trait
    shape_get_area(rec_two);

    let t = Triangle::new(6, 3);

    // using the Area trait
    shape_get_area(t);
}
