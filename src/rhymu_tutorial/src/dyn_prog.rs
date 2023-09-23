#![allow(clippy::needless_return, unused_parens, clippy::clone_on_copy)]
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
  static ref HASHMAP: Mutex<HashMap<u128, u128>> = {
    let mut map = HashMap::new();
    map.insert(0, 1);
    map.insert(1, 1);
    Mutex::new(map)
  };
}

pub fn run() {
  for n in 0..40 {
    println!("factorial[{}]: {}", n, factorial(n));
  }
}

//* Normal on DB handling */
// fn factorial(n: u128) -> u128 {
//   if n <= 1 {
//     return 1;
//   } else {
//     let _factorial = n * factorial(n - 1);
//     return _factorial;
//   }
// }

//* DB handling */
fn factorial(n: u128) -> u128 {
  let mut map = HASHMAP.lock().unwrap();
  if let Some(x) = map.get_mut(&n) {
    return x.clone();
  } else {
    drop(map);
    let _factorial = n * factorial(n - 1);
    let mut map = HASHMAP.lock().unwrap();
    map.insert(n, _factorial);
    return _factorial;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    assert_eq!(1, factorial(0));
    assert_eq!(1, factorial(1));
    assert_eq!(2, factorial(2));
    assert_eq!(6, factorial(3));
    assert_eq!(24, factorial(4));
    assert_eq!(120, factorial(5));
    assert_eq!(720, factorial(6));
    assert_eq!(5040, factorial(7));
    assert_eq!(40320, factorial(8));
    assert_eq!(362880, factorial(9));
    // assert_eq!(6, factorial(10));
  }
}
