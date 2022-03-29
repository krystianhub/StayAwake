use crate::config::ConfigError::InvalidProperty;
use serde::Deserialize;
use serde_with::{serde_as, DurationSeconds};
use std::time::Duration;
use thiserror::Error;

/// Provides default value for stayawake_interval if STAYAWAKE_INTERVAL env var is not set
fn default_stayawake_interval() -> Duration {
    Duration::from_secs(60)
}

/// Provides default value for jump_by_pixel_min if JUMP_BY_PIXEL_MIN env var is not set
fn default_jump_by_pixel_min() -> usize {
    100
}

/// Provides default value for jump_by_pixel_max if JUMP_BY_PIXEL_MAX env var is not set
fn default_jump_by_pixel_max() -> usize {
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
    #[serde(default = "default_jump_by_pixel_min")]
    pub(crate) jump_by_pixel_min: usize,
    #[serde(default = "default_jump_by_pixel_max")]
    pub(crate) jump_by_pixel_max: usize,
    #[serde(default = "default_border_pixel_size")]
    pub(crate) border_pixel_size: usize,
}

impl Config {
    /// Validates if the config is correct
    pub(crate) fn validate(&self) -> Result<(), ConfigError> {
        if self.jump_by_pixel_min == 0 {
            return Err(InvalidProperty {
                property: "jump_by_pixel_min",
                message: "jump_by_pixel_min cannot be equal to zero",
            });
        }

        if self.jump_by_pixel_max == 0 {
            return Err(InvalidProperty {
                property: "jump_by_pixel_max",
                message: "jump_by_pixel_max cannot be equal to zero",
            });
        }

        if self.border_pixel_size == 0 {
            return Err(InvalidProperty {
                property: "border_pixel_size",
                message: "border_pixel_size cannot be equal to zero",
            });
        }

        // jump by pixel min have to be lower than max
        if self.jump_by_pixel_min > self.jump_by_pixel_max {
            return Err(InvalidProperty {
                property: "jump_by_pixel_min",
                message: "jump_by_pixel_min cannot be bigger than jump_by_pixel_max",
            });
        }

        // pixel min have to be equal or bigger than border pixel size
        if self.jump_by_pixel_min >= self.border_pixel_size {
            return Err(InvalidProperty {
                property: "jump_by_pixel_min",
                message: "jump_by_pixel_min cannot be equal or bigger than border_pixel_size",
            });
        }

        // pixel max have to be equal or bigger than border pixel size
        if self.jump_by_pixel_max >= self.border_pixel_size {
            return Err(InvalidProperty {
                property: "jump_by_pixel_max",
                message: "jump_by_pixel_max cannot be equal or bigger than border_pixel_size",
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
            jump_by_pixel_min: default_jump_by_pixel_min(),
            jump_by_pixel_max: default_jump_by_pixel_max(),
            border_pixel_size: default_border_pixel_size(),
        };

        assert!(config.validate().is_ok());

        // ----------------

        let config = Config {
            stayawake_interval: default_stayawake_interval(),
            jump_by_pixel_min: 100,
            jump_by_pixel_max: 150,
            border_pixel_size: 50,
        };

        let result = config.validate();
        assert!(result.is_err());

        let result_err = result.unwrap_err();
        let InvalidProperty { property, message } = result_err;
        assert_eq!(property, "jump_by_pixel_min");
        assert_eq!(
            message,
            "jump_by_pixel_min cannot be equal or bigger than border_pixel_size"
        );

        // ----------------

        let config = Config {
            stayawake_interval: default_stayawake_interval(),
            jump_by_pixel_min: 150,
            jump_by_pixel_max: 150,
            border_pixel_size: 150,
        };

        let result = config.validate();
        assert!(result.is_err());

        let result_err = result.unwrap_err();
        let InvalidProperty { property, message } = result_err;
        assert_eq!(property, "jump_by_pixel_min");
        assert_eq!(
            message,
            "jump_by_pixel_min cannot be equal or bigger than border_pixel_size"
        );

        // ----------------

        let config = Config {
            stayawake_interval: default_stayawake_interval(),
            jump_by_pixel_min: 99,
            jump_by_pixel_max: 99,
            border_pixel_size: 100,
        };

        assert!(config.validate().is_ok());

        // ----------------

        let config = Config {
            stayawake_interval: default_stayawake_interval(),
            jump_by_pixel_min: 101,
            jump_by_pixel_max: 100,
            border_pixel_size: 150,
        };

        let result = config.validate();
        assert!(result.is_err());

        let result_err = result.unwrap_err();
        let InvalidProperty { property, message } = result_err;
        assert_eq!(property, "jump_by_pixel_min");
        assert_eq!(
            message,
            "jump_by_pixel_min cannot be bigger than jump_by_pixel_max"
        );

        // ----------------

        let config = Config {
            stayawake_interval: default_stayawake_interval(),
            jump_by_pixel_min: 0,
            jump_by_pixel_max: 1,
            border_pixel_size: 150,
        };

        let result = config.validate();
        assert!(result.is_err());

        let result_err = result.unwrap_err();
        let InvalidProperty { property, message } = result_err;
        assert_eq!(property, "jump_by_pixel_min");
        assert_eq!(message, "jump_by_pixel_min cannot be equal to zero");

        // ----------------

        let config = Config {
            stayawake_interval: default_stayawake_interval(),
            jump_by_pixel_min: 1,
            jump_by_pixel_max: 0,
            border_pixel_size: 150,
        };

        let result = config.validate();
        assert!(result.is_err());

        let result_err = result.unwrap_err();
        let InvalidProperty { property, message } = result_err;
        assert_eq!(property, "jump_by_pixel_max");
        assert_eq!(message, "jump_by_pixel_max cannot be equal to zero");

        // ----------------

        let config = Config {
            stayawake_interval: default_stayawake_interval(),
            jump_by_pixel_min: 100,
            jump_by_pixel_max: 100,
            border_pixel_size: 0,
        };

        let result = config.validate();
        assert!(result.is_err());

        let result_err = result.unwrap_err();
        let InvalidProperty { property, message } = result_err;
        assert_eq!(property, "border_pixel_size");
        assert_eq!(message, "border_pixel_size cannot be equal to zero");
    }
}
