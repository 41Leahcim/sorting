//       1: 0.000000108
//      10: 0.0000003
//     100: 0.000007752
//   1_000: 0.000576675
//  10_000: 0.054697376
// 100_000: 3.037190835
#[inline]
pub fn selection_sort<T: PartialOrd>(values: &mut [T]) {
    for i in 0..values.len() {
        let mut lowest_index = i;
        for j in i + 1..values.len() {
            if values[j] < values[lowest_index] {
                lowest_index = j;
            }
        }
        values.swap(i, lowest_index);
    }
}

#[cfg(test)]
mod test {
    use core::array;

    use super::selection_sort as sort;
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
