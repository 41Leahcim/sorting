use core::ops::Sub;

//       1: 0.000000247
//      10: 0.000000215
//     100: 0.000025838
//   1_000: 0.000530367
//  10_000: 0.0517192
// 100_000: 5.6478147199999995
pub fn bubble_sort<T: PartialOrd>(values: &mut [T]) {
    for i in 0..values.len() {
        for j in 1..values.len().sub(i) {
            if values[j - 1] > values[j] {
                values.swap(j - 1, j);
            }
        }
    }
}
