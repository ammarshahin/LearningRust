pub fn run() {
    let mut weight = String::new();
    println!("please enter your weight on Earth: ");
    std::io::stdin().read_line(&mut weight).unwrap();
    let weight: f32 = weight.trim().parse().unwrap();

    let weight_on_mars = calculate_weight_on_mars(&weight);
    println!();
    println!(
        "Your weight on Earth = {}kg\nAnd your weight on mars = {}kg",
        weight, weight_on_mars
    );
}

fn calculate_weight_on_mars(weight: &f32) -> f32 {
    (weight / 9.81) * 3.711
}
