//! Enums are types which have a few definite values
//! see : https://doc.rust-lang.org/rust-by-example/index.html

enum Movement {
    // Variants
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub fn run() {
    let avatar1 = Movement::UP;
    let avatar2 = Movement::DOWN;
    let avatar3 = Movement::LEFT;
    let avatar4 = Movement::RIGHT;
    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}

fn move_avatar(m: Movement) {
    //* match is like switch statements
    match m {
        Movement::UP => println!("Moving UP"),
        Movement::DOWN => println!("Moving DOWN"),
        Movement::LEFT => println!("Moving LEFT"),
        Movement::RIGHT => println!("Moving RIGHT"),
    }
}
