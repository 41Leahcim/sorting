pub fn linear<T: PartialEq>(data: &[T], item: &T) -> Option<usize> {
    for (index, data) in data.iter().enumerate() {
        if data == item {
            return Some(index);
        }
    }
    None
}

#[cfg(test)]
mod binary_test {
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
    fn long_array() {
        let mut data = [0; 1000];
        for (i, data) in data.iter_mut().enumerate() {
            *data = i;
        }
        assert_eq!(linear(&data, &456), Some(456));
    }

    #[test]
    fn not_found() {
        assert_eq!(linear(&[0, 1, 2], &3), None);
    }
}
