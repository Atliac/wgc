use windows::Win32::System::Performance::{QueryPerformanceCounter, QueryPerformanceFrequency};

/// Receives the current performance-counter frequency, in counts per second
pub(crate) fn qpc_frequency() -> i64 {
    let mut frequency = 0;
    unsafe {
        QueryPerformanceFrequency(&mut frequency).unwrap();
    }
    frequency
}

/// Receives the current performance-counter value
pub(crate) fn qpc_now() -> i64 {
    let mut qpc = 0;
    unsafe {
        QueryPerformanceCounter(&mut qpc).unwrap();
    }
    qpc
}

/// Returns the duration since the system booted
pub(crate) fn elapsed_since_system_boot() -> std::time::Duration {
    let qpc_now = qpc_now();
    let qpc_frequency = qpc_frequency();
    let duration = qpc_now as u128 * 1_000_000_000 / qpc_frequency as u128;
    std::time::Duration::from_nanos(duration as u64)
}
