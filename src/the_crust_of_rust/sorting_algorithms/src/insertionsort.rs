use super::*;
pub struct InsertionSort;

impl Sorter for InsertionSort {
  fn sort<T>(slice: &mut [T])
  where
    T: Ord,
  {
    for unsorted in 1..slice.len() {
      let smart_impl = false;
      if smart_impl == true {
        let mut start = unsorted;
        while start > 0 && slice[start - 1] > slice[start] {
          slice.swap(start - 1, start);
          start -= 1;
        }
      } else {
        let start = slice[..unsorted]
          .binary_search(&slice[unsorted])
          .unwrap_or_else(|i| i);
        //* is the same ===
        // let start = match slice[..unsorted].binary_search(&slice[unsorted]) {
        //     Ok(i) => i,  // will return the element position if it was found
        //     Err(i) => i, // will return the appropriate element position (to keep the slice sorted) if the element wasn't found
        // };
        slice[start..=unsorted].rotate_right(1);
      }
    }
  }
}
