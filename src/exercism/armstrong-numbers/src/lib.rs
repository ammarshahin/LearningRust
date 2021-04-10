pub fn is_armstrong_number(num: u32) -> bool {
    let digits: Vec<u32> = split_digits(num);
    let n = digits.len() as u32;
    let mut sum = 0u32;
    for dig in digits {
        sum += dig.pow(n);
    }
    sum == num
}

fn split_digits(n: u32) -> Vec<u32> {
    let mut digits = Vec::new();
    let mut num = n;
    while num != 0 {
        digits.push(num % 10);
        num /= 10;
    }
    digits.reverse();
    digits
}
