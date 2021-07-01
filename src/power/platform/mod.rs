#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "windows")]
pub use windows::*;

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "macos")]
pub use macos::*;

#[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
pub mod default;

#[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
pub use default::*;
