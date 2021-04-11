// The Drop trait     acts as a destructor for any dynamically allocated variable
// The drop trait of a variable cannot be called from the variable
// It's mainly uses are to debug where a variable is dropped

struct Creature {
    name: String,
}

impl Drop for Creature {
    fn drop(&mut self) {
        // we should leave this empty
        println!("{} is destroyed!!!", self.name);
    }
}

pub fn run() {
    let c = Creature {
        name: "eva".to_string(),
    };
    println!("{} is created", c.name);
    drop(c); // here we can manually drop (destroy) the object c by calling the drop function
             // note that the drop function only moves the variable c to it's scope and it do nothing and when it's scope ends it automatically drops c
} // here automatically calls the drop function related to all of the created objects
