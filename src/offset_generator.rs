use crate::config::Config;
use mouse_rs::types::Point;
use rand::{
    distributions::Uniform,
    prelude::{Distribution, ThreadRng},
    thread_rng, Rng,
};
use tracing::trace;

/// Random mouse position generator
pub(crate) struct OffsetGenerator {
    rng: ThreadRng,
    range: Uniform<usize>,
    config: Config,
}

impl OffsetGenerator {
    pub fn new(config: Config) -> OffsetGenerator {
        OffsetGenerator {
            rng: thread_rng(),
            range: Uniform::new_inclusive(config.jump_by_pixel_min, config.jump_by_pixel_max),
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

    /// Get randomly generated `Point` within specified boundaries
    pub(crate) fn get_random_offset_position(&mut self, init: &Point) -> Point {
        let mut x_offset = self.range.sample(&mut self.rng) as i32;
        let mut y_offset = self.range.sample(&mut self.rng) as i32;

        let start_x = self.config.init_point.x;
        let start_y = self.config.init_point.y;
        let end_x = start_x + self.config.working_area.width;
        let end_y = start_y + self.config.working_area.height;

        // Clamp initial values
        let init_x = init.x.clamp(start_x, end_x);
        let init_y = init.y.clamp(start_y, end_y);

        let is_x_near_zero = (init_x as i32 - x_offset) < start_x as i32;
        let is_y_near_zero = (init_y as i32 - y_offset) < start_y as i32;

        let is_x_near_border = (init_x + x_offset as usize) > end_x;
        let is_y_near_border = (init_y + y_offset as usize) > end_y;

        trace!(
            is_x_near_zero,
            is_y_near_zero,
            is_x_near_border,
            is_y_near_border
        );

        if !is_x_near_zero && !is_x_near_border {
            x_offset *= self.get_random_sign();
        }

        if !is_y_near_zero && !is_y_near_border {
            y_offset *= self.get_random_sign();
        }

        let x = if is_x_near_border {
            init_x as i32 - x_offset
        } else {
            init_x as i32 + x_offset
        };

        let y = if is_y_near_border {
            init_y as i32 - y_offset
        } else {
            init_y as i32 + y_offset
        };

        // Clamp final values
        let x = x.clamp(start_x as i32, end_x as i32) as usize;
        let y = y.clamp(start_y as i32, end_y as i32) as usize;

        Point { x, y }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;
    use crate::{
        config::ConfigError,
        models::{InitPoint, WorkingArea},
    };

    struct PointAsserter {
        offset_gen: OffsetGenerator,
    }

    impl PointAsserter {
        fn new(offset_gen: OffsetGenerator) -> PointAsserter {
            PointAsserter { offset_gen }
        }

        fn assert_point_eq(&mut self, start: Point, expected: Point) {
            let updated_point = self.offset_gen.get_random_offset_position(&start);
            assert_eq!(updated_point.x, expected.x);
            assert_eq!(updated_point.y, expected.y);
        }
    }

    fn setup(
        jump_by_pixel_min: usize,
        jump_by_pixel_max: usize,
        init_point: InitPoint,
        working_area: WorkingArea,
    ) -> Result<OffsetGenerator, ConfigError> {
        let test_config = Config {
            stayawake_interval: Duration::from_secs(1),
            jump_by_pixel_min,
            jump_by_pixel_max,
            init_point,
            working_area,
        };

        test_config.validate()?;

        Ok(OffsetGenerator::new(test_config))
    }

    #[test]
    fn test_get_random_offset_position() -> Result<(), ConfigError> {
        let offset_gen = setup(
            799,
            799,
            InitPoint { x: 0, y: 0 },
            WorkingArea {
                width: 800,
                height: 800,
            },
        )?;
        let mut point_asserter = PointAsserter::new(offset_gen);

        let start = Point { x: 1, y: 1 };
        let expected = Point { x: 800, y: 800 };
        point_asserter.assert_point_eq(start, expected);

        let start = Point { x: 800, y: 800 };
        let expected = Point { x: 1, y: 1 };
        point_asserter.assert_point_eq(start, expected);

        let start = Point { x: 1000, y: 1000 };
        let expected = Point { x: 1, y: 1 };
        point_asserter.assert_point_eq(start, expected);

        let start = Point { x: 0, y: 0 };
        let expected = Point { x: 799, y: 799 };
        point_asserter.assert_point_eq(start, expected);

        let start = Point { x: 800, y: 0 };
        let expected = Point { x: 1, y: 799 };
        point_asserter.assert_point_eq(start, expected);

        let start = Point { x: 0, y: 800 };
        let expected = Point { x: 799, y: 1 };
        point_asserter.assert_point_eq(start, expected);

        let start = Point { x: 0, y: 799 };
        let expected = Point { x: 799, y: 0 };
        point_asserter.assert_point_eq(start, expected);

        let start = Point { x: 0, y: 700 };
        let expected = Point { x: 799, y: 0 };
        point_asserter.assert_point_eq(start, expected);

        let start = Point { x: 2, y: 2 };
        let expected = Point { x: 0, y: 0 };
        point_asserter.assert_point_eq(start, expected);

        // -----------------

        let offset_gen = setup(
            50,
            50,
            InitPoint { x: 50, y: 50 },
            WorkingArea {
                width: 51,
                height: 51,
            },
        )?;
        let mut point_asserter = PointAsserter::new(offset_gen);

        let start = Point { x: 0, y: 0 };
        let expected = Point { x: 100, y: 100 };
        point_asserter.assert_point_eq(start, expected);

        Ok(())
    }
}
