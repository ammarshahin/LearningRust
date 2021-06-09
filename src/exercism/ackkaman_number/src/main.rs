mod lib;
use lib::*;

fn main() {
    for m in 0..5 {
        for n in 0..(16 - m) {
            println!("ackermann({}, {}) = {}", m, n, ackermann(m, n));
        }
    }
}
