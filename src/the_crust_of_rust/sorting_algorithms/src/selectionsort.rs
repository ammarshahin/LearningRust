use super::*;
pub struct SelectionSort;

impl Sorter for SelectionSort {
  fn sort<T>(slice: &mut [T])
  where
    T: Ord,
  {
    const SMART: bool = false;
    // [sorted | unsorted]
    for unsorted in 0..slice.len() {
      let mut smallest_in_unsorted;

      if SMART {
        smallest_in_unsorted = slice[unsorted..]
          .iter()
          .enumerate()
          .min_by_key(|&(_, v)| v)
          .map(|(i, _)| i + unsorted)
          .expect("Slice is empty !!!");
      } else {
        smallest_in_unsorted = unsorted;
        for i in (unsorted + 1)..slice.len() {
          if slice[i] < slice[smallest_in_unsorted] {
            smallest_in_unsorted = i;
          }
        }
      }

      // protect against unnecessary swap
      if unsorted != smallest_in_unsorted {
        slice.swap(unsorted, smallest_in_unsorted);
      }
    }
  }
}
