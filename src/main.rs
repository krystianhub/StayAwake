pub(crate) mod offset_generator;

use crate::offset_generator::OffsetGenerator;
use anyhow::{Context, Result};
use dotenv::dotenv;
use mouse_rs::Mouse;
use std::{env, time::Duration};
use tokio::time;
use tracing::{debug, error, info, trace};

const STAYAWAKE_INTERVAL_NAME: &str = "STAYAWAKE_INTERVAL";
const DEFAULT_INTERVAL_SECS: u64 = 60;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    info!("Initializing StayAwake application");

    let mut offset_gen = OffsetGenerator::new();

    let stayawake_interval: Duration = env::var(STAYAWAKE_INTERVAL_NAME)
        .map(|interval_env| interval_env.parse::<u64>().map(Duration::from_secs))
        .unwrap_or_else(|_| Ok(Duration::from_secs(DEFAULT_INTERVAL_SECS)))
        .context(format!("Cannot parse {} value", STAYAWAKE_INTERVAL_NAME))?;

    debug!(value = ?stayawake_interval, "{}", &STAYAWAKE_INTERVAL_NAME);

    let mouse = Mouse::new();
    let mut interval = time::interval(stayawake_interval);

    let get_pos_err = |err| error!(error = ?err, "Cannot get mouse position");

    info!("Initialization finished successfully");

    interval.tick().await; // Initial tick is instant

    loop {
        trace!("Loop start");

        // Grab mouse position, wait for X interval
        let pos1 = match mouse.get_position() {
            Ok(pos) => pos,
            Err(err) => {
                get_pos_err(err);
                continue;
            }
        };

        trace!("Tick started");
        interval.tick().await;
        trace!("Tick completed");

        let pos2 = match mouse.get_position() {
            Ok(pos) => pos,
            Err(err) => {
                get_pos_err(err);
                continue;
            }
        };

        trace!(?pos1, ?pos2, "Interval position results");

        // If position didn't change during the last interval, move the mouse now
        if pos1.x == pos2.x && pos1.y == pos2.y {
            let new_pos = offset_gen.get_random_offset_position(&pos1);

            trace!(
                "Movement not detected, moving mouse from {:?} to {:?}",
                &pos1,
                &new_pos
            );

            if let Err(err) = mouse.move_to(new_pos.x as i32, new_pos.y as i32) {
                error!(error = ?err, "Cannot move the mouse to a new position");
            }
        } else {
            trace!("Movement detected, not moving mouse");
        }
    }
}
