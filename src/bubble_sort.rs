#![warn(clippy::pedantic, clippy::nursery)]

//   0.000006344 for 1
//   0.000016988 for 10
//   0.000076802 for 100
//   0.001706842 for 1000
//   0.063822423 for 10000
//   5.223992289 for 100000
// 528.323772911 for 1000000
pub fn bubble_sort<T: PartialOrd>(values: &mut [T]) {
    for i in 0..values.len() {
        for j in 1..(values.len() - i) {
            if values[j - 1] > values[j] {
                values.swap(j - 1, j);
            }
        }
    }
}
