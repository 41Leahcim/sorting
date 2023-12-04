use core::cmp::Ordering;

pub fn binary<T: PartialOrd>(data: &[T], item: &T) -> Option<usize> {
    let mut a = 0;
    let mut b = data.len();
    let mut center = (a + b) / 2;
    while (a + 1..b).contains(&center) {
        match data[center].partial_cmp(item)? {
            Ordering::Less => a = center,
            Ordering::Greater => b = center,
            Ordering::Equal => return Some(center),
        };
        center = a + (b - a) / 2;
    }
    if !data.is_empty() && &data[center] == item {
        Some(center)
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::binary;

    #[test]
    fn empty() {
        assert_eq!(binary(&[], &0), None);
    }

    #[test]
    fn one_item() {
        assert_eq!(binary(&[8], &8), Some(0));
    }

    #[test]
    fn short_even_array() {
        assert_eq!(binary(&[0, 1, 2, 3, 4, 5], &4), Some(4));
    }

    #[test]
    fn short_odd_array() {
        assert_eq!(binary(&[0, 1, 2, 3, 4], &3), Some(3));
    }

    #[test]
    fn not_found() {
        assert_eq!(binary(&[0, 1, 2], &3), None);
    }

    #[cfg(feature = "alloc")]
    extern crate alloc;

    #[cfg(feature = "alloc")]
    use alloc::vec::Vec;

    #[allow(clippy::cast_possible_truncation)]
    #[test]
    fn long_array() {
        #[cfg(feature = "alloc")]
        let data = (0..u16::MAX).collect::<Vec<_>>();
        #[cfg(not(feature = "alloc"))]
        let data = {
            let mut data = [0; 1000];
            for (i, data) in data.iter_mut().enumerate() {
                *data = i as u16;
            }
            data
        };
        let value = data.len() as u16 - 1;
        assert_eq!(binary(&data, &value), Some(value as usize));
    }
}
