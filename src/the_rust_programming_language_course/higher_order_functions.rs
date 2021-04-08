pub fn run() {
    let limit = 500;
    let mut sum = 0;
    let above_limit = greater_than(limit);
    // 0..      >> will loop from 0 till infinity
    for i in 0.. {
        let sq = i * i;
        if above_limit(sq) {
            break;
        } else if sq % 2 == 0 {
            sum += sq;
        }
    }
    println!("sum = {}", sum);

    // closure fun ^__^
    let sum2 = (0..) // here we get a range from 0 to infinity  (return  RangeFrom<i32>)
        .map(|x| x * x) // here we map the range to certain functionality here we square the full range (return a   Map<RangeFrom<i32>, fn(i32) -> i32>)
        .take_while(|&x| x < limit) // here we set the limit of our range >> take value that satisfy the following condition (defined in the closure) >> (return   TakeWhile<Map<RangeFrom<i32>, fn(i32) -> i32>>)
        .filter(|x| x % 2 == 0) // here we filter the full range to satisfy the following condition (x % 2 == 0)
        .fold(0, |sum, x| sum + x); // here we fold the full range to a single value (single_value_init, closure_that_fold_the_full_range())
                                    // Try to look for every single value of thous and read it's doucs
    println!("sum = {}", sum2);
}

// the greater_than function returns a function(closure |x: u32|-> bool { y > limit })
// the signature for function return is:       impl Fn(parameter_type) -> return_type
// Fn(immutable receiver) or FnMut (mutable receiver)
fn greater_than(x: u32) -> impl Fn(u32) -> bool {
    move |y| y > x
}
