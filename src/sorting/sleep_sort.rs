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
            #[allow(clippy::shadow_reuse)]
            let tx = tx.clone();
            scope.spawn(move || {
                let delay: u64 = value.clone().try_into().unwrap_or(u64::MAX);
                thread::sleep(Duration::from_millis(delay * length));
                tx.send(value).ok();
            });
        }
        for (target, source) in values.iter_mut().zip(rx) {
            *target = source;
        }
    });
}
