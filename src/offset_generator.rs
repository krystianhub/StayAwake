use mouse_rs::types::Point;
use rand::{
    distributions::Uniform,
    prelude::{Distribution, ThreadRng},
    thread_rng, Rng,
};
use tracing::trace;

const OFFSET_PIXEL_MIN: usize = 50;
const OFFSET_PIXEL_MAX: usize = 100;

pub(crate) struct OffsetGenerator {
    rng: ThreadRng,
    range: Uniform<usize>,
}

impl OffsetGenerator {
    pub fn new() -> OffsetGenerator {
        OffsetGenerator {
            rng: thread_rng(),
            range: Uniform::new_inclusive(OFFSET_PIXEL_MIN, OFFSET_PIXEL_MAX),
        }
    }

    fn get_random_sign(&mut self) -> i32 {
        if self.rng.gen() {
            1
        } else {
            -1
        }
    }

    pub(crate) fn get_random_offset_position(&mut self, init: &Point) -> Point {
        let is_near_zero = init.x < OFFSET_PIXEL_MAX || init.y < OFFSET_PIXEL_MAX;

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
