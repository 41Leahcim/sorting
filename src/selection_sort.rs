#![warn(clippy::pedantic, clippy::nursery)]

use std::ops::Add;

//      1: 0.000000337
//     10: 0.000000416
//    100: 0.000008076
//   1000: 0.000371379
//  10000: 0.038134269
// 100000: 4.418165212
pub fn selection_sort<T: PartialOrd>(values: &mut [T]) {
    (0..values.len()).for_each(|i|{
        let mut lowest_index = i;
        (i.add(1)..values.len()).for_each(|j|{
            if values[j] < values[lowest_index] {
                lowest_index = j;
            }
        });
        values.swap(i, lowest_index);
    });
}
