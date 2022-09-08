pub mod bubble_sort;
pub mod insertion_sort;
pub mod merge_sort;
pub mod selection_sort;

use std::time::SystemTime;
use bubble_sort::bubble_sort;
use insertion_sort::insertion_sort;
use merge_sort::merge_sort;
use selection_sort::selection_sort;

#[allow(dead_code)]
enum Algorithm{
    BubbleSort,
    InsertionSort,
    MergeSort,
    SelectionSort
}

const ARRAY_LENGTH: usize = 100;

fn main() {
    let start = SystemTime::now();
    let algorithm = Algorithm::BubbleSort;
    let mut values: Vec<u32> = Vec::new();
    values.reserve(ARRAY_LENGTH);

    for i in 0..ARRAY_LENGTH{
        values.push((ARRAY_LENGTH - i) as u32);
    }

    match algorithm{
        Algorithm::BubbleSort => bubble_sort(&mut values),
        Algorithm::InsertionSort => insertion_sort(&mut values),
        Algorithm::MergeSort => merge_sort(&mut values),
        Algorithm::SelectionSort => selection_sort(&mut values)
    }

    eprintln!("{:?}", values);
    if let Ok(elapsed) = start.elapsed(){
        eprintln!("{}", elapsed.as_secs_f64());
    }
}
