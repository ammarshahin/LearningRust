#[derive(PartialEq, Debug)]
enum Type {
  Odd,
  Even,
}

fn main() {
  for i in (1_000_000..10_000_000).step_by(2) {
    match goldbach_numbers(i) {
      Ok((n1, n2)) => {
        println!("{}= {} + {}", i, n1, n2);
      }
      Err(num_type) => match num_type {
        Type::Even => {
          println!("{}: is Even, Theory Wrong!!!!", i)
        }
        Type::Odd => println!("{}: is Odd, Doesn't have to be tested!", i),
      },
    };
  }
}

fn goldbach_numbers(n: u128) -> Result<(u128, u128), Type> {
  if is_even(n).is_err() {
    return Err(Type::Odd);
  }

  for i in 2..((n / 2) + 1) {
    if is_prime(i).is_ok() && is_prime(n - i).is_ok() {
      return Ok((i, n - i));
    }
  }

  Err(Type::Even)
}

fn is_prime(n: u128) -> Result<(), ()> {
  // If the number is less than or equal to 1, it is not prime
  if n <= 1 {
    return Err(());
  }

  // Check for divisors from 2 to n-1
  for i in 2..((n / 2) + 1) {
    // If n is divisible by any number in this range, it is not prime
    if n % i == 0 {
      return Err(());
    }
  }

  // If no divisors are found, it is prime
  Ok(())
}

fn is_even(n: u128) -> Result<(), ()> {
  if n % 2 == 0 {
    return Ok(());
  }
  Err(())
}

mod tests {
  #[test]
  /// Test when the input number is odd, then the goldbach_numbers function should return Err()  
  fn test_goldbach_odd() {
    for i in (1..100).step_by(2) {
      assert_eq!(super::goldbach_numbers(i), Err(super::Type::Odd));
    }
  }

  #[test]
  /// Test if the goldbach conjecture holds up to a specific number
  fn test_goldbach() {
    for i in (4..100).step_by(2) {
      assert!(super::goldbach_numbers(i).is_ok());
    }
  }

  #[test]
  fn test_if_prime() {
    assert!(super::is_prime(0).is_err());
    assert!(super::is_prime(1).is_err());
    assert!(super::is_prime(2).is_ok());
    assert!(super::is_prime(3).is_ok());
    assert!(super::is_prime(4).is_err());
    assert!(super::is_prime(5).is_ok());
  }

  #[test]
  fn test_if_prime_tmp() {
    let x = 1030509;
    // match super::is_prime(x) {
    // Ok(_) => panic!("{}: is a prime", x),
    // Err(_) => panic!("{}: isn't a prime", x),
    // }
  }

  #[test]
  fn test_goldbach_tmp() {
    let x = 1030510;
    // match super::goldbach_numbers(x) {
    //   Ok((n1, n2)) => panic!("{}: {} + {}", x, n1, n2),
    //   Err(_) => panic!("{}: isn't a prime", x),
    // }
  }
}
