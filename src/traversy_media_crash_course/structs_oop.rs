//! Struct as class

//* Define the elements of the class (Attributes)
struct Rec {
    width: u8,
    hight: u8,
}

//* Define the methods of the class (functions)
impl Rec {
    //* Explicit Constructor
    fn new(w: u8, h: u8) -> Rec {
        return Rec { width: w, hight: h };
    }

    //* Class getters
    fn get_width(&self) -> u8 {
        return self.width;
    }

    fn get_hight(&self) -> u8 {
        return self.hight;
    }

    //* Class setters
    fn change_hight(&mut self, h: u8) {
        self.hight = h;
    }

    fn change_width(&mut self, w: u8) {
        self.width = w;
    }

    //* methods
    fn get_area(&self) -> u8 {
        return self.hight * self.width;
    }
}

pub fn run() {
    //* create an object
    //* using the implicit constructor
    let mut c1 = Rec { hight: 6, width: 4 };

    //* using the explicit constructor
    let _c2 = Rec::new(6, 4);

    //* change the internal attributes using the setters
    c1.change_width(8);
    c1.change_hight(8);

    println!(
        "area of the rectangle {} * {} = {}",
        _c2.get_width(),
        _c2.get_hight(),
        _c2.get_area()
    );
}
