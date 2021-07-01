#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "windows")]
pub use windows::*;

#[cfg(not(target_os = "windows"))]
pub mod default;

#[cfg(not(target_os = "windows"))]
pub use default::*;
