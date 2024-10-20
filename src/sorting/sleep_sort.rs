extern crate std;
use core::{fmt::Debug, time::Duration};
use std::{sync::mpsc::channel, thread};

#[inline]
pub fn sleep_sort<T: TryInto<u64> + Clone + Send>(values: &mut [T])
where
    T::Error: Debug,
{
    let Ok(length) = u64::try_from(values.len()) else {
        return;
    };
    thread::scope(|scope| {
        let (tx, rx) = channel::<T>();
        for value in values.iter().cloned() {
            #[expect(clippy::shadow_reuse)]
            let tx = tx.clone();
            scope.spawn(move || {
                let delay: u64 = value.clone().try_into().unwrap_or(u64::MAX);
                thread::sleep(Duration::from_millis(delay * length));
                tx.send(value)
            });
        }
        for (target, source) in values.iter_mut().zip(rx) {
            *target = source;
        }
    });
}

#[cfg(test)]
mod test {
    use core::array;

    use super::sleep_sort as sort;
    use crate::sorting::is_sorted;

    #[test]
    fn sorts() {
        const ARRAY_LENGTH: usize = 10;
        let mut data: [usize; ARRAY_LENGTH] = array::from_fn(|i| ARRAY_LENGTH - i - 1);
        assert!(!is_sorted(&data));
        sort(&mut data);
        assert!(is_sorted(&data));
    }
}
