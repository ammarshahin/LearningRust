/************************** traits *********************************/
trait Area {
    fn get_area(&self) -> u8;
}

// generic functions that takes a generic type T
fn shape_get_area<T: Area>(item: T) {
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
    let c2 = Rectangle::new(6, 4);

    // access the struct elements
    println!(
        "area of the rectangle {} * {} = {}",
        c2.get_width(),
        c2.get_height(),
        c2.get_area()
    );

    // using the implicit constructor
    let mut c1 = Rectangle {
        height: 6,
        width: 4,
    };
    //* change the internal attributes using the setters
    c1.change_width(8);
    c1.change_height(8);

    // using the Area trait
    shape_get_area(c1);

    let t = Triangle::new(6, 3);

    // using the Area trait
    shape_get_area(t);
}
