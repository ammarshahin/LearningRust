//* Ownership Rules
//*     - Each value in Rust has a variable that’s called its owner.
//*     - There can only be one owner at a time.
//*     - When the owner goes out of scope, the value will be dropped.

pub fn run() {
  let mut s = String::from("hello"); // string literal (mutable variable that can be modified)
  s.push_str(", world!"); // push_str() appends a literal to a String
  println!("{}", s); // This will print `hello, world!`

  let s2 = s; //* Rust considers s to no longer be valid and, therefore, Rust doesn’t need to free anything when s goes out of scope. And s2 is now the valid variable
              // println!("{}, world!", s); // >> this will give an error because s has been "moved" or "shallow copied" to s2

  //* Deep copy
  let s3 = s2.clone();
  println!("{}", s2); // >> this will work because s2 has been "cloned" or "deep copy" to s3 and not "shallow copied"

  //* Ownership and Functions
  takes_ownership(s3); // s3 value moves into the function. And so is no longer valid here

  let x = 5; // x comes into scope
  makes_copy(x); // x would move into the function, but i32 implements the Copy trait, so it’s okay to still use x afterward
  println!("{}", x);

  let _s4 = gives_ownership(); // gives_ownership moves its return value into s4
  let _s5 = String::from("hello"); // s5 comes into scope
  let _s6 = takes_and_gives_back(s2); // s5 is moved into takes_and_gives_back, which also moves its return value into s6
} // x goes out of scope, then s. But because s's value was moved, nothing special happens. And, s3 goes out of scope and is dropped. s2 goes out of scope but was moved, so nothing happens. s1 goes out of scope and is dropped.

fn takes_ownership(some_string: String) {
  // some_string comes into scope
  println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) {
  // some_integer comes into scope
  println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
  // gives_ownership will move its return value into the function that calls it
  let some_string = String::from("hello"); // some_string comes into scope
  some_string // some_string is returned and moves out to the calling function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String {
  // a_string comes into scope
  a_string // a_string is returned and moves out to the calling function
}
