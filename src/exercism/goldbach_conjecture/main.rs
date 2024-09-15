#[derive(PartialEq, Debug)]
enum Type {
  Odd,
  Even,
}

// impl PartialEq for Type {
//   fn eq(&self, other: &Self) -> bool {
//     core::mem::discriminant(self) == core::mem::discriminant(other)
//   }
// }

fn main() {
  for i in (4..1000).step_by(2) {
    match goldbach_numbers(i) {
      Ok((n1, n2)) => {
        println!("{}= {} + {}", i, n1, n2);
      }
      Err(num_type) => match num_type {
        Type::Even => {
          println!("Number is Even and doesn't have a goldbach_numbers!!!, Theory Wrong!!!!")
        }
        Type::Odd => println!("Number is Odd, Doesn't have to be tested!"),
      },
    };
  }
}

fn goldbach_numbers(n: u32) -> Result<(u32, u32), Type> {
  let n1: u32;
  let n2: u32;

  if is_even(n).is_err() {
    return Err(Type::Odd);
  }

  let max = (n / 2) + 1;
  for i in 2..max {
    if is_prime(i).is_ok() && is_prime(n - i).is_ok() {
      n1 = i;
      n2 = n - i;
      return Ok((n1, n2));
    }
  }

  Err(Type::Even)
}

fn is_prime(n: u32) -> Result<(), ()> {
  // If the number is less than or equal to 1, it is not prime
  if n <= 1 {
    return Err(());
  }

  // Check for divisors from 2 to n-1
  let max = (n / 2) + 1;
  for i in 2..max {
    // If n is divisible by any number in this range, it is not prime
    if n % i == 0 {
      return Err(());
    }
  }

  // If no divisors are found, it is prime
  Ok(())
}

fn is_even(n: u32) -> Result<(), ()> {
  if n % 2 == 0 {
    return Ok(());
  }
  Err(())
}

mod tests {
  #[test]
  ///! Test when the input number is odd, then the goldbach_numbers function should return Err()  
  fn test_goldbach_odd() {
    for i in (1..100).step_by(2) {
      assert_eq!(super::goldbach_numbers(i), Err(super::Type::Odd));
    }
  }

  #[test]
  fn test_goldbach() {
    for i in (4..100000).step_by(2) {
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
}
