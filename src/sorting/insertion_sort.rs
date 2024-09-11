//       1: 0.000000109
//      10: 0.000000192
//     100: 0.000005459
//   1_000: 0.000544856
//  10_000: 0.056226938
// 100_000: 5.892911743
#[inline]
pub fn insertion_sort<T: PartialOrd>(values: &mut [T]) {
    for j in 1..values.len() {
        for i in (1..=j).rev() {
            if values[i] >= values[i - 1] {
                break;
            }
            values.swap(i, i - 1);
        }
    }
}

#[cfg(test)]
mod test {
    use core::array;

    use super::insertion_sort as sort;
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
