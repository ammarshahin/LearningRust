#![allow(dead_code)]

pub fn run() {
  let mut hands = Hands::new("banana", "apple");
  hands.report();
  hands.juggle();
  hands.report();
}

#[derive(Clone, Debug, PartialEq)]
struct Hands {
  left: Option<String>,
  right: Option<String>,
}

impl Hands {
  fn new(left_holds: &str, right_holds: &str) -> Self {
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

fn report_item(item: &Option<String>, arg: &str) {
  match item {
    Some(what) => {
      println!("{} hand is holding {}", arg, what);
    }
    _ => {
      println!("{} hand is not holding anything!!", arg);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn create_hands() {
    let mut hands = Hands::new("banana", "apple");
    let expected_hands = Hands::new("apple", "banana");
    hands.report();
    hands.juggle();
    hands.report();
    assert_eq!(hands, expected_hands);
  }
}
