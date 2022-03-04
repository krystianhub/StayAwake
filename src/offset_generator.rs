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

        let mut x_offset = self.range.sample(&mut self.rng) as i32;
        let mut y_offset = self.range.sample(&mut self.rng) as i32;

        trace!(is_near_zero);

        if !is_near_zero {
            x_offset *= self.get_random_sign();
            y_offset *= self.get_random_sign();
        }

        let x = init.x as i32 + x_offset;
        let y = init.y as i32 + y_offset;

        Point {
            x: x as usize,
            y: y as usize,
        }
    }
}
