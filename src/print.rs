
pub fn run()
{   
    println!("Hello, world!");
    println!("My name is {}", "ammar");
    println!("My name is {} and I'm {}", "ammar", 26);
    println!("My name is {0} and I'm {1}. {0} loves to code in rust", "ammar", 26);
    

    // Named Arguments
  println!(
    "{name} likes to play {activity}",
    name = "ammar",
    activity = "video Games"
  );

  // Placeholder traits
  println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

  // Placeholder for debug trait
  println!("{:?}", (12, true, "hello"));

  // Basic math
  println!("10 + 10 = {}", 10 + 10);

}