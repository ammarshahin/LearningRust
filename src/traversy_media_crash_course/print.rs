pub fn run() {
    println!("Hello, world!");

    // default formatter
    println!("My name is {}", "ammar");
    println!("My name is {} and I'm {}", "ammar", 26);

    // numbered formatter
    println!(
        "My name is {0} and I'm {1}. {0} loves to code in rust",
        "ammar", 26
    );

    // Named Arguments formatter
    println!(
        "{name} likes to play {activity}",
        name = "ammar",
        activity = "video Games"
    );

    // float formatter
    println!("{:0.2}", 1.2354);

    // Placeholder traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait [Array formatter]
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);

    // format
    let my_name = format!("John Doe");

    println!("my name is {}", my_name);
}
