#![allow(dead_code)]

pub fn run() {
  let mut hands = Hands::new("bnanna", "apple");
  hands.report();
  hands.juggle();
  hands.report();
}

struct Hands {
  left: Item,
  right: Item,
}

impl Hands {
  fn new(left_holds: &str, right_holds: &str) -> Self {
    Self {
      left: Item {
        what: left_holds.to_owned(),
        present: true,
      },
      right: Item {
        what: right_holds.to_owned(),
        present: true,
      },
    }
  }

  fn report(&self) {
    Item::report_item(&self.left, "Left");
    Item::report_item(&self.right, "Right");
  }

  fn juggle(&mut self) {
    println!("Let's juggle!!!");
    let temp = self.left.clone();
    self.left = self.right.clone();
    self.right = temp;
  }
}

#[derive(Clone)]
struct Item {
  what: String,
  present: bool,
}

impl Item {
  fn new(what: String, present: bool) -> Self {
    Self { what, present }
  }

  fn report_item(&self, arg: &str) {
    if self.present {
      println!("{} hand is holding {}", arg, self.what);
    } else {
      println!("{} hand is not holding anything!!", arg);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn create_hands() {
    let mut hands = Hands::new("bnanna", "apple");
    hands.report();
    hands.juggle();
    hands.report();
    assert_eq!(hands.right.what, "bnanna".to_owned());
    assert_eq!(hands.left.what, "apple".to_owned());
  }
}
