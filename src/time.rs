use arcdps::Event;
use std::time::Duration;
use windows::Win32::Media::timeGetTime;

/// Retrieves the current system time.
///
/// The system time is the number of milliseconds elapsed since Windows was started.
#[inline]
pub fn now() -> u64 {
    unsafe { timeGetTime() as u64 }
}

/// Calculates the time elapsed between the timestamp and now.
#[inline]
pub fn since(time: u64) -> Duration {
    let now = Duration::from_millis(now());
    let time = Duration::from_millis(time);
    now.saturating_sub(time)
}

/// Calculates the time elapsed between an event happening and now.
#[inline]
pub fn since_event(event: &Event) -> Duration {
    since(event.time)
}
