//       1: 0.000000247
//      10: 0.000000215
//     100: 0.000025838
//   1_000: 0.000530367
//  10_000: 0.0517192
// 100_000: 5.6478147199999995
#[inline]
pub fn bubble_sort<T: PartialOrd>(values: &mut [T]) {
    for i in 0..values.len() {
        for j in 1..values.len() - i {
            if values[j - 1] > values[j] {
                values.swap(j - 1, j);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use core::array;

    use super::bubble_sort as sort;
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
