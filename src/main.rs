#![warn(clippy::pedantic, clippy::nursery)]
#![feature(is_sorted)]

pub mod bubble_sort;
pub mod insertion_sort;
pub mod merge_sort;
pub mod selection_sort;
pub mod bogo_sort;

use bubble_sort::bubble_sort;
use insertion_sort::insertion_sort;
use merge_sort::merge_sort;
use selection_sort::selection_sort;
use bogo_sort::bogo_sort;
use std::time::Instant;

#[allow(dead_code)]
enum Algorithm {
    Bubble,
    Insertion,
    Merge,
    Selection,
    Bogo
}

const ARRAY_LENGTH: usize = 16;

fn main() {
    let start = Instant::now();
    let algorithm = Algorithm::Bogo;
    let mut values = Vec::new();
    values.reserve(ARRAY_LENGTH);

    for i in 0..ARRAY_LENGTH {
        values.push(ARRAY_LENGTH - i);
    }

    match algorithm {
        Algorithm::Bubble => bubble_sort(&mut values),
        Algorithm::Insertion => insertion_sort(&mut values),
        Algorithm::Merge => merge_sort(&mut values),
        Algorithm::Selection => selection_sort(&mut values),
        Algorithm::Bogo => bogo_sort(&mut values)
    }

    eprintln!("{values:?}");
    eprintln!("{}", start.elapsed().as_secs_f64());
}
