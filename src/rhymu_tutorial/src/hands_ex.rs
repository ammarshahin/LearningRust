pub fn run() {
  let mut hands = Hands::new("bnanna", "apple");
  hands.report();
  hands.juggle();
  hands.report();
}

pub struct Hands {
  left: Item,
  right: Item,
}

impl Hands {
  pub fn new(left_holds: &str, right_holds: &str) -> Self {
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

  pub fn report(&self) {
    Item::report_item(&self.left, "Left");
    Item::report_item(&self.right, "Right");
  }

  pub fn juggle(&mut self) {
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
  #[allow(dead_code)]
  fn new(what: String, present: bool) -> Self {
    Self { what, present }
  }

  fn report_item(hand: &Item, arg: &str) {
    if hand.present {
      println!("{} hand is holding {}", arg, hand.what);
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
