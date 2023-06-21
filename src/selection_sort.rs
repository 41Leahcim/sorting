use std::ops::Add;

//       1: 0.000000108
//      10: 0.0000003
//     100: 0.000007752
//   1_000: 0.000576675
//  10_000: 0.054697376
// 100_000: 3.037190835
pub fn selection_sort<T: PartialOrd>(values: &mut [T]) {
    for i in 0..values.len() {
        let mut lowest_index = i;
        for j in i.add(1)..values.len() {
            if values[j] < values[lowest_index] {
                lowest_index = j;
            }
        }
        values.swap(i, lowest_index);
    }
}
