use crate::config::Config;
use mouse_rs::types::Point;
use rand::{
    distributions::Uniform,
    prelude::{Distribution, ThreadRng},
    thread_rng, Rng,
};
use tracing::trace;

/// Random mouse position generator
pub(crate) struct OffsetGenerator<'a> {
    rng: ThreadRng,
    range: Uniform<usize>,
    config: &'a Config,
}

impl OffsetGenerator<'_> {
    pub fn new(config: &Config) -> OffsetGenerator {
        OffsetGenerator {
            rng: thread_rng(),
            range: Uniform::new_inclusive(config.offset_pixel_min, config.offset_pixel_max),
            config,
        }
    }

    fn get_random_sign(&mut self) -> i32 {
        if self.rng.gen() {
            1
        } else {
            -1
        }
    }

    /// Get randomly generated `Point`
    pub(crate) fn get_random_offset_position(&mut self, init: &Point) -> Point {
        let is_near_zero =
            init.x < self.config.offset_pixel_min || init.y < self.config.offset_pixel_max;

        let is_near_border =
            init.x < self.config.border_pixel_size || init.y < self.config.border_pixel_size;

        let mut x_offset = self.range.sample(&mut self.rng) as i32;
        let mut y_offset = self.range.sample(&mut self.rng) as i32;

        trace!(is_near_zero, is_near_border);

        if !is_near_zero && !is_near_border {
            x_offset *= self.get_random_sign();
            y_offset *= self.get_random_sign();
        }

        let x;
        let y;

        if is_near_border {
            x = init.x as i32 - x_offset;
            y = init.y as i32 - y_offset;
        } else {
            x = init.x as i32 + x_offset;
            y = init.y as i32 + y_offset;
        }

        Point {
            x: x as usize,
            y: y as usize,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[test]
    fn test_get_random_offset_position() {
        let test_config = Config {
            stayawake_interval: Duration::from_secs(15),
            offset_pixel_min: 100,
            offset_pixel_max: 150,
            border_pixel_size: 800,
        };

        // TODO: test get_random_offset_position
        // - Test different configs and so forth
    }
}
