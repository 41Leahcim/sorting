extern crate alloc;

use alloc::vec::Vec;

// Merges the values of both halves of the values slice in order.
// This is the most important step in mergesort.
fn merge<T: PartialOrd + Copy>(values: &mut [T], buffer: &mut Vec<T>) {
    // Determine the middle of the array
    let middle = values.len() / 2;

    // Create 1 index that starts at the start of the array and another starting in
    // the middle
    let (mut i, mut j) = (0, middle);

    // Clear the buffer
    buffer.clear();

    // Add the values in order to the buffer
    while i < middle && j < values.len() {
        if values[i] < values[j] {
            buffer.push(values[i]);
            i += 1;
        } else {
            buffer.push(values[j]);
            j += 1;
        }
    }

    // Add the rest of the values to the buffer
    buffer.extend_from_slice(&values[i..middle]);
    buffer.extend_from_slice(&values[j..]);

    // Copy the values from the buffer to the correct part of the array
    for (target, source) in values.iter_mut().zip(buffer.iter()) {
        *target = *source;
    }
}

//           1: 0.000000406
//          10: 0.000000801
//         100: 0.000003501
//       1_000: 0.000026241
//      10_000: 0.000438492
//     100_000: 0.003813203
//   1_000_000: 0.038404551
//  10_000_000: 0.396210257
// 100_000_000: 4.210718009
/// # Panics
/// Will eventually panic when sorting more than 2¹²⁸ values.
#[inline]
pub fn merge_sort<T: PartialOrd + Copy>(values: &mut [T]) {
    // Create a vector for the index ranges
    //let expected_max_length = power_of_two_index(values.len()) * 2 + 1;
    //let mut ranges = Vec::with_capacity(expected_max_length);
    let mut ranges = Vec::with_capacity(257);

    // Create a vector to store values when merging
    let mut buffer = Vec::with_capacity(values.len());

    // Add the first range
    ranges.push(0..values.len());

    // Store the current length as the previous length
    let mut previous_length = values.len();

    // Continue sorting until all ranges have been sorted
    while let Some(range) = ranges.pop() {
        // Check the length of the current range
        let current_length = range.len();
        match current_length {
            // If the range only contains 0 or 1 values, those values are sorted
            0 | 1 => {}

            // If the range contains 2 values and they are in the right order, they are sorted
            2 if values[range.start] < values[range.end - 1] => {}

            // If it contains 2 values in the wrong order, swap them
            2 => values.swap(range.start, range.end - 1),

            // If the previous length was greater than the current length, split the range
            length if previous_length >= length => {
                let middle = range.start + length / 2;
                ranges.extend_from_slice(&[range.clone(), range.start..middle, middle..range.end]);
            }

            // Otherwise, merge the range
            _ => merge(&mut values[range], &mut buffer),
        }
        // Store the current length as the previous length
        previous_length = current_length;
    }
}

#[cfg(test)]
mod test {
    use core::array;

    use super::merge_sort as sort;
    use crate::sorting::is_sorted;

    #[test]
    fn sorts() {
        const ARRAY_LENGTH: usize = 1_000;
        let mut data: [usize; ARRAY_LENGTH] = array::from_fn(|i| ARRAY_LENGTH - i - 1);
        assert!(!is_sorted(&data));
        sort(&mut data);
        assert!(is_sorted(&data));
    }
}
