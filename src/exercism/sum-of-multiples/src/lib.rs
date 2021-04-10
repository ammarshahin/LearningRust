pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut sum = 0;
    for i in 1..limit {
        for n in factors {
            if (*n != 0) && ((i % *n) == 0) {
                sum += i;
                break;
            }
        }
    }
    sum
}
