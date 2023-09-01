mod lib;
use lib::*;

fn main() {
  let x = 5;
  for m in 0..5 {
    for n in 0..(16 - m) {
      println!("ackermann({}, {}) = {}", m, n, ackermann(m, n));
    }
  }
}
