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
  let sum2 = (0..) // get a range from 0 to infinity
    .map(|x| x * x) // map the range to certain functionality >>  square the full range
    .take_while(|&x| x < limit) // set the limit of our range >> take value that satisfy the following condition (defined in the closure)
    .filter(|x| x % 2 == 0) // filter the full range to satisfy the following condition (x % 2 == 0)
    .fold(0, |sum, x| sum + x); // fold the full range to a single value (single_value_init, closure_that_fold_the_full_range())
  println!("sum = {}", sum2);
}

// the greater_than function returns a function(closure |x: u32|-> bool { y > limit })
// the signature for function return is:       impl Fn(parameter_type) -> return_type
// Fn(immutable receiver) or FnMut (mutable receiver)
fn greater_than(x: u32) -> impl Fn(u32) -> bool {
  move |y| y > x
}
