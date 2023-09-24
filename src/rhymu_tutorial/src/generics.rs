#![allow(dead_code)]
use std::fmt::{Debug, Display};

pub fn run() {
  let mut hands = Hands::new(&Fruit::Apple, &Fruit::Banana);
  hands.report();
  hands.juggle();
  hands.report();
}

#[derive(Clone, Debug, PartialEq)]
struct Hands<T> {
  left: Option<T>,
  right: Option<T>,
}

impl<T> Hands<T>
where
  T: Display + Clone,
{
  fn new(left_holds: &T, right_holds: &T) -> Self {
    Self {
      left: Some(left_holds.to_owned()),
      right: Some(right_holds.to_owned()),
    }
  }

  fn report(&self) {
    report_item(&self.left, "Left");
    report_item(&self.right, "Right");
  }

  fn juggle(&mut self) {
    println!("Let's juggle!!!");
    let temp = self.left.clone();
    self.left = self.right.clone();
    self.right = temp;
  }
}

fn report_item<T: Display>(item: &Option<T>, arg: &str) {
  match item {
    Some(what) => {
      println!("{} hand is holding {}", arg, what);
    }
    _ => {
      println!("{} hand is not holding anything!!", arg);
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
enum Fruit {
  Apple,
  Banana,
  Kiwi,
}

impl Display for Fruit {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Fruit::Apple => write!(f, "an Apple"),
      Fruit::Banana => write!(f, "a Banana"),
      Fruit::Kiwi => write!(f, "a Kiwi"),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn create_hands_string() {
    let mut hands = Hands::new(&"banana".to_owned(), &"apple".to_owned());
    let expected_hands = Hands::new(&"apple".to_owned(), &"banana".to_owned());
    hands.report();
    hands.juggle();
    hands.report();
    assert_eq!(hands, expected_hands);
  }

  #[test]
  fn create_hands_int() {
    let mut hands = Hands::new(&10, &20);
    let expected_hands = Hands::new(&20, &10);
    hands.report();
    hands.juggle();
    hands.report();
    assert_eq!(hands, expected_hands);
  }

  #[test]
  fn create_hands_float() {
    let mut hands = Hands::new(&1.2, &2.4);
    let expected_hands = Hands::new(&2.4, &1.2);
    hands.report();
    hands.juggle();
    hands.report();
    assert_eq!(hands, expected_hands);
  }

  #[test]
  fn create_hands_fruit() {
    let mut hands = Hands::new(&Fruit::Apple, &Fruit::Banana);
    let expected_hands = Hands::new(&Fruit::Banana, &Fruit::Apple);
    hands.report();
    hands.juggle();
    hands.report();
    assert_eq!(hands, expected_hands);
  }
}
