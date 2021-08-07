#![allow(dead_code, unused_variables)]

pub fn run() {
  // UTF-8 string
  let s1 = "hello World"; // s1 is a string slice >> &'static str >>'static is a life time operator meaning it will life throughout the full program
                          // S1 is invalid to manipulate or change single chars

  // loop over &str
  for i in s1.chars() {
    print!("{}", i);
  }

  println!();

  // access individual letters and chars
  if let Some(first_char) = s1.chars().nth(0) {
    println!("the first chars of the string is {}", first_char);
  }

  // String >> heap allocated construct
  let mut s2 = String::new(); // s2 is dynamic string that can be expanded and shrink
  let mut a = 'a' as u8; // here we converted 'a' to a u8 to iterate over it
  while a <= 'z' as u8 {
    s2.push(a as char); // here we convert it back to char to push it to the string
    a += 1; // increment the a variable >> this cannot be done on chars
  }

  // to push a full string at the end of another
  s2.push_str(" John Doe!!");
  println!("s2 = {}", s2);

  // Concatenation is done >> str3(String) = str2(String) + str1(&str)
  let s3 = s2 + s1; // here we concatenate the strings s2 and s1 >> this possible because s2 is String and s1 is &str >> if we reverse the add (s1 + s2) will not work
  println!("s3 = {}", s3);

  let s4 = "any &str".to_string(); // this will convert any &str string to String
  println!("s4 = {}", s4); // original string
  println!("s4 = {}", s4.replace("any", "Any")); // this will return a new string that is manipulated from the previous s4 >> s4 is still the same

  let name = "John Doe";
  let greetings = format!("hi, my name is {}", name); // format! macro companies strings and return a String
  println!("{}", greetings);
}
