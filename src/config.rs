use serde::Deserialize;
use serde_with::{serde_as, DurationSeconds};
use std::time::Duration;

/// Provides default value for stayawake_interval if STAYAWAKE_INTERVAL env var is not set
fn default_stayawake_interval() -> Duration {
    Duration::from_secs(60)
}

/// Provides default value for offset_pixel_min if OFFSET_PIXEL_MIN env var is not set
fn default_offset_pixel_min() -> usize {
    5
}

/// Provides default value for offset_pixel_max if OFFSET_PIXEL_MAX env var is not set
fn default_offset_pixel_max() -> usize {
    15
}

/// Configuration struct (.env file)
#[serde_as]
#[derive(Deserialize, Debug)]
pub(crate) struct Config {
    #[serde_as(as = "DurationSeconds")]
    #[serde(default = "default_stayawake_interval")]
    pub(crate) stayawake_interval: Duration,
    #[serde(default = "default_offset_pixel_min")]
    pub(crate) offset_pixel_min: usize,
    #[serde(default = "default_offset_pixel_max")]
    pub(crate) offset_pixel_max: usize,
}
