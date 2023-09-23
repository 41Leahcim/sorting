use core::cmp::Ordering;

pub fn binary<T: PartialOrd>(data: &[T], item: &T) -> Option<usize> {
    let mut a = 0;
    let mut b = data.len();
    let mut center = (a + b) / 2;
    while a != center && center != b {
        match data[center].partial_cmp(item).unwrap() {
            Ordering::Less => a = center,
            Ordering::Greater => b = center,
            Ordering::Equal => return Some(center),
        };
        center = (a + b) / 2;
    }
    if !data.is_empty() && &data[center] == item {
        Some(center)
    } else {
        None
    }
}

#[cfg(test)]
mod binary_test {
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
    fn long_array() {
        let mut data = [0; 1000];
        for (i, data) in data.iter_mut().enumerate() {
            *data = i;
        }
        assert_eq!(binary(&data, &456), Some(456));
    }

    #[test]
    fn not_found() {
        assert_eq!(binary(&[0, 1, 2], &3), None);
    }
}
