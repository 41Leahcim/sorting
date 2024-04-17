#[inline]
pub fn linear<T: PartialEq>(data: &[T], item: &T) -> Option<usize> {
    for (index, value) in data.iter().enumerate() {
        if value == item {
            return Some(index);
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::linear;

    #[test]
    fn empty() {
        assert_eq!(linear(&[], &0), None);
    }

    #[test]
    fn one_item() {
        assert_eq!(linear(&[8], &8), Some(0));
    }

    #[test]
    fn short_even_array() {
        assert_eq!(linear(&[0, 1, 2, 3, 4, 5], &4), Some(4));
    }

    #[test]
    fn short_odd_array() {
        assert_eq!(linear(&[0, 1, 2, 3, 4], &3), Some(3));
    }

    #[test]
    fn not_found() {
        assert_eq!(linear(&[0, 1, 2], &3), None);
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
        assert_eq!(linear(&data, &value), Some(value as usize));
    }
}
