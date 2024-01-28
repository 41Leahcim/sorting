use std::{
    fmt::Write,
    time::{Duration, Instant},
};

#[cfg(feature = "alloc")]
use algorithms::sorting::merge_sort::merge_sort;
#[cfg(feature = "std")]
use algorithms::sorting::sleep_sort::sleep_sort;
use algorithms::sorting::{
    bubble_sort::bubble_sort, insertion_sort::insertion_sort, is_sorted,
    selection_sort::selection_sort,
};

use rayon::prelude::{IntoParallelIterator, ParallelIterator};

const MAX_DURATION: Duration = Duration::from_secs(1);

fn measure_performance<T>(func: impl Fn(T), arg: T) -> Duration {
    let start = Instant::now();
    func(arg);
    start.elapsed()
}

fn sorting_test(function: fn(&mut [u32]), name: &str) {
    let mut performance = Duration::from_secs(0);
    let mut max_value = 1;
    let mut output = String::new();
    writeln!(&mut output, "{name}").unwrap();
    while performance < MAX_DURATION {
        let mut data = (0..max_value).rev().collect::<Vec<u32>>();
        performance = measure_performance(function, &mut data);
        assert!(
            is_sorted(&data),
            "{name} failed to sort array of size {max_value}: {data:?}"
        );
        writeln!(&mut output, "{max_value}: {}", performance.as_secs_f64()).unwrap();
        max_value *= 10;
    }
    println!("{output}");
}

fn main() {
    #[allow(clippy::type_complexity)]
    let sorting: Vec<(fn(&mut [u32]), &str)> = vec![
        (bubble_sort, "bubble_sort"),
        (insertion_sort, "insertion_sort"),
        #[cfg(feature = "alloc")]
        (merge_sort, "merge_sort"),
        (selection_sort, "selection_sort"),
        #[cfg(feature = "std")]
        (sleep_sort, "sleep_sort"),
    ];
    sorting
        .into_par_iter()
        .for_each(|(function, name)| sorting_test(function, name));

    #[allow(clippy::type_complexity)]
    let searching: [(fn(&[usize], &usize) -> Option<usize>, &str); 2] = [
        (algorithms::searching::binary::binary, "binary_search"),
        (algorithms::searching::linear::linear, "linear_search"),
    ];

    #[allow(clippy::type_complexity)]
    searching.into_par_iter().for_each(|(function, name)| {
        let mut performance = Duration::from_secs(0);
        let mut max_value = 1;
        let mut output = String::new();
        writeln!(&mut output, "{name}").unwrap();
        while performance < MAX_DURATION && max_value < 1_000_000_000 {
            let data = (0..max_value).collect::<Vec<_>>();
            let start = Instant::now();
            function(&data, &max_value);
            performance = start.elapsed();
            assert_eq!(
                function(&data, &max_value),
                None,
                "{name} found a value that was not in the array"
            );
            assert_eq!(
                function(&data, &(max_value - 1)),
                Some(max_value - 1),
                "{name} didn't find a value that was in the array"
            );
            assert!(is_sorted(&data));
            writeln!(&mut output, "{max_value}: {}", performance.as_secs_f64()).unwrap();
            max_value *= 10;
        }
        println!("{output}");
    })
}
