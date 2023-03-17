#![warn(clippy::pedantic, clippy::nursery)]

pub mod bubble_sort;
pub mod insertion_sort;
pub mod merge_sort;
pub mod selection_sort;

use bubble_sort::bubble_sort;
use insertion_sort::insertion_sort;
use merge_sort::merge_sort;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use selection_sort::selection_sort;
use std::time::Instant;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
enum Algorithm {
    Bubble,
    Insertion,
    Merge,
    Selection,
}

fn sort_performance_test(array_length: usize, algorithm: Algorithm, output: &mut String) -> f64{
    let start = Instant::now();
    let mut values = (0..array_length).rev().collect::<Vec<usize>>();

    match algorithm {
        Algorithm::Bubble => bubble_sort(&mut values),
        Algorithm::Insertion => insertion_sort(&mut values),
        Algorithm::Merge => merge_sort(&mut values),
        Algorithm::Selection => selection_sort(&mut values),
    }

    let performance = start.elapsed().as_secs_f64();
    *output = format!("{output}\n{array_length:9}: {performance}");
    performance
}

fn main() {
    [Algorithm::Bubble, Algorithm::Insertion, Algorithm::Merge, Algorithm::Selection].into_par_iter().for_each(|algorithm|{
        let mut buffer = String::new();
        let mut array_length = 1;
        while sort_performance_test(array_length, algorithm, &mut buffer) < 1.0{
            array_length *= 10;
        }
        eprintln!("{algorithm:?}{buffer}\n");
    });
}
