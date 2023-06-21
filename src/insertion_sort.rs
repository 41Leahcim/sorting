//       1: 0.000000182
//      10: 0.000000487
//     100: 0.000012729
//   1_000: 0.001155257
//  10_000: 0.104986252
// 100_000: 7.813983133
pub fn insertion_sort<T: PartialOrd>(values: &mut [T]) {
    let mut i = 1;
    while i < values.len() {
        if values[i] < values[i - 1] {
            values.swap(i, i - 1);
            if i > 1 {
                i -= 1;
            } else {
                i += 1;
            }
        } else {
            i += 1;
        }
    }
}
