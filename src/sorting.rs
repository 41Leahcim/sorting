pub mod bubble_sort;
pub mod insertion_sort;
#[cfg(feature = "alloc")]
pub mod merge_sort;
pub mod selection_sort;
#[cfg(feature = "std")]
pub mod sleep_sort;

/// Checks whether a slice is sorted.
/// Empty slices and slices only containing one value are always sorted.
/// Only works for low to high sorted slices.
pub fn is_sorted<T: PartialOrd>(values: &[T]) -> bool {
    for i in 1..values.len() {
        if values[i - 1] > values[i] {
            return false;
        }
    }
    true
}
