#![allow(dead_code)]
use std::fmt::Debug;

pub fn run() {
  let mut hands = Hands::new(&Fruit::Apple, &Fruit::Bnanna);
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
  T: Debug + Clone,
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

fn report_item<T: Debug>(item: &Option<T>, arg: &str) {
  match item {
    Some(what) => {
      println!("{} hand is holding a {:?}", arg, what);
    }
    _ => {
      println!("{} hand is not holding anything!!", arg);
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
enum Fruit {
  Apple,
  Bnanna,
  Kiwi,
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn create_hands_string() {
    let mut hands = Hands::new(&"bnanna".to_owned(), &"apple".to_owned());
    let expected_hands = Hands::new(&"apple".to_owned(), &"bnanna".to_owned());
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
    let mut hands = Hands::new(&Fruit::Apple, &Fruit::Bnanna);
    let expected_hands = Hands::new(&Fruit::Bnanna, &Fruit::Apple);
    hands.report();
    hands.juggle();
    hands.report();
    assert_eq!(hands, expected_hands);
  }
}
