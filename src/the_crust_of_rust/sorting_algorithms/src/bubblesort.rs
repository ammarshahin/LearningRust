use super::*;
pub struct BubbleSort {}

impl Sorter for BubbleSort {
  fn sort<T>(slice: &mut [T])
  where
    T: Ord,
  {
    let mut swapped = true;
    while swapped {
      swapped = false;
      for i in 0..(slice.len() - 1) {
        if slice[i] > slice[i + 1] {
          slice.swap(i, i + 1);
          swapped = true;
        }
      }
    }
  }
}
