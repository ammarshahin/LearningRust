mod bubblesort;
mod insertionsort;

pub trait Sorter {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord;
}

pub fn sort<T, S>(slice: &mut [T])
where
    T: Ord,
    S: Sorter,
{
    S::sort(slice);
}

#[cfg(test)]
mod tests {
    use super::*;
    use bubblesort::BubbleSort;
    use insertionsort::InsertionSort;
    /****** healthy check *******/
    struct StdSorter;
    impl Sorter for StdSorter {
        fn sort<T>(slice: &mut [T])
        where
            T: Ord,
        {
            slice.sort();
        }
    }
    #[test]
    fn sorter_works() {
        let mut w = vec![2, 8, 1, 99, 27];
        StdSorter::sort(&mut w);
        assert_eq!(w, vec![1, 2, 8, 27, 99]);
    }

    #[test]
    fn bubble_sort_works() {
        let mut w1 = vec![2, 8, 1, 99, 27, 3];
        BubbleSort::sort(&mut w1);
        assert_eq!(w1, vec![1, 2, 3, 8, 27, 99]);

        let mut w2 = vec![287, 8, 15, 98, 27, 3];
        BubbleSort::sort(&mut w2);
        assert_eq!(w2, vec![3, 8, 15, 27, 98, 287]);
    }

    #[test]
    fn insertion_sort_works() {
        let mut w1 = vec![2, 8, 1, 99, 27, 3];
        InsertionSort::sort(&mut w1);
        assert_eq!(w1, vec![1, 2, 3, 8, 27, 99]);

        let mut w2 = vec![287, 8, 15, 98, 27, 3];
        InsertionSort::sort(&mut w2);
        assert_eq!(w2, vec![3, 8, 15, 27, 98, 287]);

        let mut w3 = vec![1250, 858, 14, 2, 8, 3, 1250];
        InsertionSort::sort(&mut w3);
        assert_eq!(w3, vec![2, 3, 8, 14, 858, 1250, 1250]);
    }
}
