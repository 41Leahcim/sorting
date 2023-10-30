extern crate alloc;

use alloc::borrow::ToOwned;

//           1: 0.000000144
//          10: 0.000000441
//         100: 0.000026834
//       1_000: 0.000046258
//      10_000: 0.000353535
//     100_000: 0.003675381
//   1_000_000: 0.039050525
//  10_000_000: 0.492102457
// 100_000_000: 5.874551763
pub fn merge_sort<T: PartialOrd + Copy>(array: &mut [T]) {
    if array.len() == 2 && array[0] > array[1] {
        array.swap(0, 1);
    } else if array.len() > 2 {
        let (array2, array3) = array.split_at(array.len() / 2);
        let mut array2 = array2.to_owned();
        let mut array3 = array3.to_owned();
        merge_sort(&mut array2);
        merge_sort(&mut array3);
        let mut i = 0;
        let mut j = 0;
        let mut k = 0;
        while i < array2.len() && j < array3.len() {
            if array2[i] < array3[j] {
                array[k] = array2[i];
                i += 1;
            } else {
                array[k] = array3[j];
                j += 1;
            }
            k += 1;
        }
        for (a, b) in array
            .iter_mut()
            .skip(k)
            .zip(array2.into_iter().skip(i).chain(array3.into_iter().skip(j)))
        {
            *a = b;
        }
    }
}
