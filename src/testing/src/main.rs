fn main() {
  println!("Hello, world! {}", fun());
}

fn fun() -> i32 {
  5
}

#[test]
fn test_fun() {
  assert_eq!(fun(), 5);
}
