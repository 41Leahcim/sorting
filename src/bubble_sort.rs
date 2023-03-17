#![warn(clippy::pedantic, clippy::nursery)]

use std::ops::Sub;

//      1: 0.000000549
//     10: 0.000000314
//    100: 0.000008364
//   1000: 0.000801116
//  10000: 0.07983358
// 100000: 6.064803106
pub fn bubble_sort<T: PartialOrd>(values: &mut [T]) {
    (0..values.len()).for_each(|i|{
        (1..values.len().sub(i)).for_each(|j|{
            if values[j - 1] > values[j] {
                values.swap(j - 1, j);
            }
        });
    });
}
