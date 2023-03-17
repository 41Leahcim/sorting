#![warn(clippy::pedantic, clippy::nursery)]

use std::ops::Sub;

//         1: 0.000000871
//        10: 0.000001066
//       100: 0.000009116
//      1000: 0.000054972
//     10000: 0.000397916
//    100000: 0.004889183
//   1000000: 0.054042774
//  10000000: 0.656803313
// 100000000: 8.23213366
pub fn merge_sort<T: PartialOrd + Copy>(array: &mut [T]) {
    if array.len() == 2 && array[0] > array[1] {
        array.swap(0, 1);
    } else if array.len() > 2{
        let half_size = array.len() / 2;
        let mut array2 = (0..half_size).map(|i| array[i]).collect::<Vec<T>>();
        let mut array3 = (0..array.len().sub(half_size)).map(|i| array[half_size + i]).collect::<Vec<T>>();
        merge_sort(&mut array2);
        merge_sort(&mut array3);
        let mut i = 0;
        let mut j = 0;
        let mut k = 0;
        while i < array2.len() && j < array3.len() {
            if array2[i] < array3[j] {
                array[k] = array2[i];
                i += 1;
            } else {
                array[k] = array3[j];
                j += 1;
            }
            k += 1;
        }
        while i < array2.len() {
            array[k] = array2[i];
            i += 1;
            k += 1;
        }
        while j < array3.len() {
            array[k] = array3[j];
            j += 1;
            k += 1;
        }
    }
}
