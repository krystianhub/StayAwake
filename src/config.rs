use crate::config::ConfigError::InvalidProperty;
use serde::Deserialize;
use serde_with::{serde_as, DurationSeconds};
use std::time::Duration;
use thiserror::Error;

/// Provides default value for stayawake_interval if STAYAWAKE_INTERVAL env var is not set
fn default_stayawake_interval() -> Duration {
    Duration::from_secs(60)
}

/// Provides default value for offset_pixel_min if OFFSET_PIXEL_MIN env var is not set
fn default_offset_pixel_min() -> usize {
    100
}

/// Provides default value for offset_pixel_max if OFFSET_PIXEL_MAX env var is not set
fn default_offset_pixel_max() -> usize {
    150
}

/// Provides default value for border_pixel_size if BORDER_PIXEL_SIZE env var is not set
fn default_border_pixel_size() -> usize {
    800
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Configuration property `{property}` is invalid: {message}")]
    InvalidProperty {
        property: &'static str,
        message: &'static str,
    },
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
    #[serde(default = "default_border_pixel_size")]
    pub(crate) border_pixel_size: usize,
}

impl Config {
    /// Validates if the config is correct
    pub(crate) fn validate(&self) -> Result<(), ConfigError> {
        // offset pixel min have to be lower than max
        if self.offset_pixel_min >= self.offset_pixel_max {
            return Err(InvalidProperty {
                property: "offset_pixel_min",
                message: "offset_pixel_min cannot be equal or bigger than offset_pixel_max",
            });
        }

        // pixel min have to be bigger than border pixel size
        if self.offset_pixel_min >= self.border_pixel_size {
            return Err(InvalidProperty {
                property: "offset_pixel_min",
                message: "offset_pixel_min cannot be equal or bigger than border_pixel_size",
            });
        }

        // pixel max have to be bigger than border pixel size
        if self.offset_pixel_max >= self.border_pixel_size {
            return Err(InvalidProperty {
                property: "offset_pixel_max",
                message: "offset_pixel_max cannot be equal or bigger than border_pixel_size",
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_validate() {
        let config = Config {
            stayawake_interval: default_stayawake_interval(),
            offset_pixel_min: default_offset_pixel_min(),
            offset_pixel_max: default_offset_pixel_max(),
            border_pixel_size: default_border_pixel_size(),
        };

        assert!(config.validate().is_ok());

        // ----------------

        let config = Config {
            stayawake_interval: default_stayawake_interval(),
            offset_pixel_min: 100,
            offset_pixel_max: 150,
            border_pixel_size: 50,
        };

        let result = config.validate();
        assert!(result.is_err());

        let result_err = result.unwrap_err();
        let InvalidProperty { property, message } = result_err;
        assert_eq!(property, "offset_pixel_min");
        assert_eq!(
            message,
            "offset_pixel_min cannot be equal or bigger than border_pixel_size"
        );

        // TODO: config unit tests (finish them)
    }
}
