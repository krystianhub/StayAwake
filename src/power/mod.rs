pub use wasmer_enumset::EnumSet;
use wasmer_enumset::EnumSetType;

#[cfg(target_os = "windows")]
const POWER_DEFAULT_MESSAGE: &str = "StayAwake program is running";

/// Platform-specific types
pub mod platform;

/// Common trait implemented by all platform-specific inhibition managers
///
/// Produces [`Lock`]s, which inhibit specific power management operations.
///
/// [`Lock`]: ./trait.Lock.html
pub trait InhibitionManager {
    type Error: std::error::Error;
    type Lock: Lock;

    /// Produces a new [`Lock`] that inhibits the given operations
    /// # Parameters
    ///
    /// - `types`: The types of operations to inhibit
    ///
    /// [`Lock`]: ./trait.Lock.html
    fn lock(&self, types: EnumSet<LockType>) -> Result<Self::Lock, Self::Error>;
}

/// Inhibits a particular power management operation until the `Lock` is dropped.
pub trait Lock: Send {}

#[derive(Debug, EnumSetType)]
pub enum LockType {
    /// Automatic suspension (managed by the system idle timer)
    AutomaticSuspend,
    /// Manual suspension
    ManualSuspend,
}

/// Constructs a new [`InhibitionManager`] for the current platform.
pub fn manager(
) -> Result<platform::InhibitionManager, <platform::InhibitionManager as InhibitionManager>::Error>
{
    platform::InhibitionManager::new()
}
