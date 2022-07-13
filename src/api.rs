use arcdps::CombatEvent;
use std::time::Duration;
use windows::Win32::Media::timeGetTime;

/// Calculates the time difference (deltatime) between an event happening and now.
pub fn calc_delta(event: &CombatEvent) -> Duration {
    let now = Duration::from_millis(unsafe { timeGetTime() } as u64);
    let event_time = Duration::from_millis(event.time);
    now.saturating_sub(event_time)
}
