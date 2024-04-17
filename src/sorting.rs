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
#[inline]
pub fn is_sorted<T: PartialOrd>(values: &[T]) -> bool {
    !(1..values.len()).any(|i| values[i - 1] > values[i])
}
