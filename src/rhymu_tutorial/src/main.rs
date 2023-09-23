#[macro_use]
extern crate lazy_static;
mod borrowing;
mod dyn_prog;
mod generics;

fn main() {
  // borrowing::run();
  // dyn_prog::run();
  generics::run();
}
