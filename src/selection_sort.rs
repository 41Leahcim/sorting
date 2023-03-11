#![warn(clippy::pedantic, clippy::nursery)]

// 0.000004410 for 1
// 0.000011003 for 10
// 0.000103706 for 100
// 0.001355115 for 1000
// 0.063606111 for 10000
// 2.666211606 for 100000
pub fn selection_sort<T: PartialOrd>(values: &mut [T]) {
    for i in 0..values.len() {
        let mut lowest_index = i;
        for j in (i + 1)..values.len() {
            if values[j] < values[lowest_index] {
                lowest_index = j;
            }
        }
        values.swap(i, lowest_index);
    }
}
