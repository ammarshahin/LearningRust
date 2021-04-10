pub fn factors(n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();
    let mut num = n;
    let mut loop_counter = 2;

    while loop_counter <= num {
        if num % loop_counter == 0 {
            factors.push(loop_counter);
            num = num / loop_counter;
            if num == 1 {
                break;
            }
            loop_counter = 1;
        }
        loop_counter += 1;
    }
    factors
}
