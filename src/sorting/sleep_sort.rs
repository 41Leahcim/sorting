extern crate std;
use core::{fmt::Debug, time::Duration};
use std::{sync::mpsc::channel, thread};

pub fn sleep_sort<T: TryInto<u64> + Copy + Send>(values: &mut [T])
where
    T::Error: Debug,
{
    let Ok(length) = u64::try_from(values.len()) else {
        return;
    };
    thread::scope(|s| {
        let (tx, rx) = channel::<T>();
        for value in values.iter().copied() {
            let tx = tx.clone();
            s.spawn(move || {
                let delay: u64 = value.try_into().unwrap_or(u64::MAX);
                thread::sleep(Duration::from_millis(delay * length));
                tx.send(value).ok();
            });
        }
        for (target, source) in values.iter_mut().zip(rx) {
            *target = source;
        }
    });
}
