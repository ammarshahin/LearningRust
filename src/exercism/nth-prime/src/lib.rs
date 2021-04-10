pub fn nth(n: u32) -> u32 {
    let mut prime_counter = 0;
    'a: for num in 0.. {
        if is_prime(num) {
            if prime_counter == n {
                prime_counter = num;
                break 'a;
            }
            prime_counter += 1;
        }
    }
    return prime_counter;
}

fn is_prime(n: u32) -> bool {
    // assume 0 and 1 aren't prime
    if n == 0 || n == 1 {
        return false;
    }

    let limit = (n as f64).sqrt() as u32;
    for i in 2..=limit {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}
