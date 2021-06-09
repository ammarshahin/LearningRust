pub fn ackermann(m: u64, n: u64) -> u64 {
    match (m, n) {
        (0, n) => n + 1,
        (m, 0) => ackermann(m - 1, 1),
        (m, n) => ackermann(m - 1, ackermann(m, n - 1)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut ans_vec: Vec<u64> = Vec::new();
        ans_vec.push(ackermann(0, 0));
        ans_vec.push(ackermann(1, 0));
        ans_vec.push(ackermann(0, 1));
        ans_vec.push(ackermann(1, 1));
        assert_eq!(ans_vec, vec![1, 2, 2, 3]);
    }
}
