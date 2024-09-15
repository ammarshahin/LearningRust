fn main() {
  let mut max = 0;
  for i in (4..1000).step_by(2) {
    let (ret, n1, n2) = goldbach_numbers(i);
    println!("{}({}): {} + {}", i, ret, n1, n2);

    if n1 <= n2 && n1 > max {
      max = n1;
    }
  }
  println!("Max: {}", max);
}

fn goldbach_numbers(n: u32) -> (bool, u32, u32) {
  let mut n1: u32 = 0;
  let mut n2: u32 = 0;
  let mut ret = false;

  if !is_even(n) {
    return (ret, n1, n2);
  }

  let max = (n / 2) + 1;
  for i in 2..max {
    if is_prime(i) && is_prime(n - i) {
      n1 = i;
      n2 = n - i;
      ret = true;
      break;
    }
  }

  (ret, n1, n2)
}

fn is_prime(n: u32) -> bool {
  let mut ret = true;
  // If the number is less than or equal to 1, it is not prime
  if n <= 1 {
    ret = false;
  }

  // Check for divisors from 2 to n-1
  let max = (n / 2) + 1;
  for i in 2..max {
    // If n is divisible by any number in this range, it is not prime
    if n % i == 0 {
      ret = false;
      break;
    }
  }

  // If no divisors are found, it is prime
  ret
}

fn is_even(n: u32) -> bool {
  if n % 2 == 0 {
    return true;
  }
  false
}

mod tests {
  #[test]
  fn test_goldbach_odd() {
    for i in (1..100).step_by(2) {
      let (ret, n1, n2) = super::goldbach_numbers(i);
      assert_eq!(ret, false);
      assert_eq!(n1, 0);
      assert_eq!(n2, 0);
    }
  }

  #[test]
  fn test_goldbach_not_prime() {
    for n in (4..100000).step_by(2) {
      let (ret, _, _) = super::goldbach_numbers(n);
      assert_eq!(ret, true);
    }
  }

  #[test]
  fn test_if_prime_0() {
    assert_eq!(super::is_prime(0), false);
  }

  #[test]
  fn test_if_prime_1() {
    assert_eq!(super::is_prime(1), false);
  }

  #[test]
  fn test_if_prime_2() {
    assert_eq!(super::is_prime(2), true);
  }

  #[test]
  fn test_if_prime_3() {
    assert_eq!(super::is_prime(3), true);
  }

  #[test]
  fn test_if_prime_4() {
    assert_eq!(super::is_prime(4), false);
  }
}
