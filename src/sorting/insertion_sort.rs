//       1: 0.000000109
//      10: 0.000000192
//     100: 0.000005459
//   1_000: 0.000544856
//  10_000: 0.056226938
// 100_000: 5.892911743
pub fn insertion_sort<T: PartialOrd>(values: &mut [T]) {
    for i in 1..values.len() {
        for i in (1..=i).rev() {
            if values[i] >= values[i - 1] {
                break;
            }
            values.swap(i, i - 1);
        }
    }
}

#[cfg(test)]
mod test {
    use super::insertion_sort as sort;
    use crate::sorting::is_sorted;

    #[test]
    fn sorts() {
        let mut data = [0; 1000];
        data.iter_mut()
            .rev()
            .enumerate()
            .for_each(|(value, item)| *item = value);
        assert!(!is_sorted(&data));
        sort(&mut data);
        assert!(is_sorted(&data));
    }
}
