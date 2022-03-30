use crate::{
    config::ConfigError::InvalidProperty,
    models::{InitPoint, WorkingArea},
};
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

/// Provides default value for init_point if INIT_POINT env var is not set
fn default_init_point() -> InitPoint {
    InitPoint { x: 0, y: 0 }
}

/// Provides default value for working_area if WORKING_AREA env var is not set
fn default_working_area() -> WorkingArea {
    WorkingArea {
        width: 1024,
        height: 768,
    }
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
    #[serde_as(as = "DurationSeconds<u64>")]
    #[serde(default = "default_stayawake_interval")]
    pub(crate) stayawake_interval: Duration,
    #[serde(default = "default_jump_by_pixel_min")]
    pub(crate) jump_by_pixel_min: usize,
    #[serde(default = "default_jump_by_pixel_max")]
    pub(crate) jump_by_pixel_max: usize,
    #[serde(default = "default_init_point")]
    pub(crate) init_point: InitPoint,
    #[serde(default = "default_working_area")]
    pub(crate) working_area: WorkingArea,
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

        if self.working_area.width == 0 || self.working_area.height == 0 {
            return Err(InvalidProperty {
                property: "working_area",
                message: "working_area height or/and width cannot be equal to zero",
            });
        }

        // jump by pixel min have to be smaller than max
        if self.jump_by_pixel_min > self.jump_by_pixel_max {
            return Err(InvalidProperty {
                property: "jump_by_pixel_min",
                message: "jump_by_pixel_min cannot be bigger than jump_by_pixel_max",
            });
        }

        // working area size have to be bigger than jump by pixel max
        if self.working_area.width <= self.jump_by_pixel_max
            || self.working_area.height <= self.jump_by_pixel_max
        {
            return Err(InvalidProperty {
                property: "working_area",
                message: "working_area cannot be equal or smaller than jump_by_pixel_max",
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
            init_point: default_init_point(),
            working_area: default_working_area(),
        };

        assert!(config.validate().is_ok());

        // ----------------

        let config = Config {
            stayawake_interval: default_stayawake_interval(),
            jump_by_pixel_min: 100,
            jump_by_pixel_max: 150,
            init_point: InitPoint { x: 0, y: 0 },
            working_area: WorkingArea {
                width: 50,
                height: 500,
            },
        };

        let result = config.validate();
        assert!(result.is_err());

        let result_err = result.unwrap_err();
        let InvalidProperty { property, message } = result_err;
        assert_eq!(property, "working_area");
        assert_eq!(
            message,
            "working_area cannot be equal or smaller than jump_by_pixel_max"
        );

        // ----------------

        let config = Config {
            stayawake_interval: default_stayawake_interval(),
            jump_by_pixel_min: 100,
            jump_by_pixel_max: 150,
            init_point: InitPoint { x: 0, y: 0 },
            working_area: WorkingArea {
                width: 500,
                height: 50,
            },
        };

        let result = config.validate();
        assert!(result.is_err());

        let result_err = result.unwrap_err();
        let InvalidProperty { property, message } = result_err;
        assert_eq!(property, "working_area");
        assert_eq!(
            message,
            "working_area cannot be equal or smaller than jump_by_pixel_max"
        );

        // ----------------

        let config = Config {
            stayawake_interval: default_stayawake_interval(),
            jump_by_pixel_min: 150,
            jump_by_pixel_max: 150,
            init_point: InitPoint { x: 0, y: 0 },
            working_area: WorkingArea {
                width: 150,
                height: 150,
            },
        };

        let result = config.validate();
        assert!(result.is_err());

        let result_err = result.unwrap_err();
        let InvalidProperty { property, message } = result_err;
        assert_eq!(property, "working_area");
        assert_eq!(
            message,
            "working_area cannot be equal or smaller than jump_by_pixel_max"
        );

        // ----------------

        let config = Config {
            stayawake_interval: default_stayawake_interval(),
            jump_by_pixel_min: 99,
            jump_by_pixel_max: 99,
            init_point: InitPoint { x: 0, y: 0 },
            working_area: WorkingArea {
                width: 100,
                height: 100,
            },
        };

        assert!(config.validate().is_ok());

        // ----------------

        let config = Config {
            stayawake_interval: default_stayawake_interval(),
            jump_by_pixel_min: 99,
            jump_by_pixel_max: 99,
            init_point: InitPoint { x: 50, y: 50 },
            working_area: WorkingArea {
                width: 100,
                height: 100,
            },
        };

        assert!(config.validate().is_ok());

        // ----------------

        let config = Config {
            stayawake_interval: default_stayawake_interval(),
            jump_by_pixel_min: 101,
            jump_by_pixel_max: 100,
            init_point: InitPoint { x: 0, y: 0 },
            working_area: WorkingArea {
                width: 150,
                height: 150,
            },
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
            init_point: InitPoint { x: 0, y: 0 },
            working_area: WorkingArea {
                width: 150,
                height: 150,
            },
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
            init_point: InitPoint { x: 0, y: 0 },
            working_area: WorkingArea {
                width: 150,
                height: 150,
            },
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
            init_point: InitPoint { x: 0, y: 0 },
            working_area: WorkingArea {
                width: 0,
                height: 0,
            },
        };

        let result = config.validate();
        assert!(result.is_err());

        let result_err = result.unwrap_err();
        let InvalidProperty { property, message } = result_err;
        assert_eq!(property, "working_area");
        assert_eq!(
            message,
            "working_area height or/and width cannot be equal to zero"
        );

        // ----------------

        let config = Config {
            stayawake_interval: default_stayawake_interval(),
            jump_by_pixel_min: 100,
            jump_by_pixel_max: 100,
            init_point: InitPoint { x: 0, y: 0 },
            working_area: WorkingArea {
                width: 0,
                height: 150,
            },
        };

        let result = config.validate();
        assert!(result.is_err());

        let result_err = result.unwrap_err();
        let InvalidProperty { property, message } = result_err;
        assert_eq!(property, "working_area");
        assert_eq!(
            message,
            "working_area height or/and width cannot be equal to zero"
        );
    }
}
