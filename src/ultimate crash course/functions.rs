pub fn run() {
    let width = 4;
    let height = 7;
    let depth = 10;
    let area = area_of(width, height);
    println!("Area is {}", area);
    println!("Volume is {}", volume(width, height, depth));
}

fn volume(width: i32, height: i32, depth: i32) -> i32 {
    return width * height * depth;
}

fn area_of(x: i32, y: i32) -> i32 {
    return x * y;
}
