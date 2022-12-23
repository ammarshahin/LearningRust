fn main() {
  println!("{}", fun());
}

fn fun() -> String {
  "Hello, world! ".to_string()
}

#[test]
fn test_fun() {
  assert_eq!(fun(), "Hello, world! ");
}
