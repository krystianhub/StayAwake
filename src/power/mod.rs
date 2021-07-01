#[cfg(any(target_os = "windows", target_os = "macos"))]
const POWER_DEFAULT_MESSAGE: &str = "StayAwake program is running";

/// Platform-specific types
pub mod platform;

pub trait Lock: Send {
    type Error: std::error::Error;
    type Lock: Lock;

    fn new(&self) -> Result<Self::Lock, Self::Error>;
}

/// Constructs a new [`Lock`] for the current platform.
pub fn lock() -> Result<platform::Lock, <platform::Lock as Lock>::Error> {
    platform::Lock::new()
}
