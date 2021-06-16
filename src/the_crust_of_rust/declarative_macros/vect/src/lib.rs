// + zero or more repartition
// * means any number of elements grater than zero
// ? means 0 or 1 of this pattern
#![allow(unused_macros)]

/****************************************************** trait implementation ****************************************/
trait MaxValue {
    fn max_value() -> Self;
}

macro_rules! max_impl {
    ($t:ty) => {
        impl $crate::MaxValue for $t {
            fn max_value() -> Self {
                <$t>::MAX
            }
        }
    };
}

max_impl!(u32);
max_impl!(i32);
max_impl!(u64);
max_impl!(i64);
max_impl!(f32);
max_impl!(f64);

#[test]
fn max_value_test() {
    assert_eq!(u32::max_value(), 4294967295);
}

/************************************************ vec macro ****************************************************/
#[macro_export]
macro_rules! avec {
    // the + sign means one or more repetition of the first argument separated by a ,
    ($($element:expr),* $(,)?) => {{
        #[allow(unused_mut)]
        let mut vs = Vec::new();
        // $(statement)*  => repeat the statements between the brackets as many times as the pattern repeats (for every element input)
        $(vs.push($element);)*
        vs
    }};

    // one element entered repeated a count times
    ($element:expr; $count:expr) => {{
        let value = $element;
        let count = $count;
        let mut vs = Vec::with_capacity(count);
        // vs.extend(std::iter::repeat(value).take(count));
        vs.resize(count,value); // more efficient
        vs
    }};
}

#[test]
fn one_element_repeated() {
    let x = avec![42; 16];
    assert_eq!(x.len(), 16);
    assert_eq!(
        x,
        vec![42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42]
    );
}

#[test]
fn empty_vec() {
    let x: Vec<u32> = avec![];
    assert!(x.is_empty())
}

#[test]
fn has_one_element() {
    let x = avec![42];
    assert_eq!(x.len(), 1);
    assert_eq!(x[0], 42);
}

#[test]
fn has_multiple_elements() {
    let x = avec![
        42, 43, 42, 43, 42, 43, 42, 43, 42, 43, 42, 43, 42, 43, 42, 43, 42, 43, 42, 43, 42, 43, 42,
        43, 42, 43, 42, 43, 42, 43, 42, 43, 42, 43, 42, 43,
    ];
    assert_eq!(x.len(), 36);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 43);
    assert_eq!(
        x,
        vec![
            42, 43, 42, 43, 42, 43, 42, 43, 42, 43, 42, 43, 42, 43, 42, 43, 42, 43, 42, 43, 42, 43,
            42, 43, 42, 43, 42, 43, 42, 43, 42, 43, 42, 43, 42, 43,
        ]
    );
}
