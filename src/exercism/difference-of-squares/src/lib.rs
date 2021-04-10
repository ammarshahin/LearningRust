pub fn square_of_sum(n: u32) -> u32 {
    let mut sum = 0;
    for n in 0..=n {
        sum += n;
    }
    (sum as f64).sqrt() as u32
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut sum = 0;
    for n in 0..=n {
        sum += (n as f64).sqrt() as u32;
    }
    sum
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
