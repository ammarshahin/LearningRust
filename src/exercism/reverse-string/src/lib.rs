extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let word = input
        // Split the string into an Iterator of &strs, where each element is an extended grapheme cluster.
        .graphemes(true)
        // Reverse the order of the grapheme iterator.
        .rev()
        // Collect all the chars into a new owned String.
        .collect();
    word
}
