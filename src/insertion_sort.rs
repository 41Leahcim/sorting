#![warn(clippy::pedantic, clippy::nursery)]

//      1: 0.000000473
//     10: 0.000000528
//    100: 0.000013211
//   1000: 0.001210332
//  10000: 0.129037339
// 100000: 9.038658903
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
